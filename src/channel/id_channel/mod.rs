mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;
use id::REGISTRY;

use alloc::sync::Arc;
use core::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    pin::Pin,
    task::Waker,
};
use futures::{
    channel::mpsc::{unbounded, SendError, UnboundedReceiver, UnboundedSender},
    future::ok,
    task::{Context as FContext, Poll},
    Future as IFuture, FutureExt, Sink as ISink, SinkExt, Stream, StreamExt, TryFutureExt,
};
use parking_lot::RawMutex;
use serde::{de::DeserializeOwned, Serialize};
use setwaker::SetWaker;
use std::{collections::HashMap, sync::Mutex};
use thiserror::Error;

use crate::{
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle, Waiter},
    core::spawn,
    kind::{Fallible, Future, Sink},
    Kind, SerdeAny, Target,
};

use super::{ChannelError, Shim as IShim};

#[derive(Clone)]
struct SinkStages<T> {
    ready: T,
    flush: T,
    close: T,
}

impl<T> SinkStages<T> {
    fn new(cons: impl Fn() -> T) -> Self {
        SinkStages {
            ready: cons(),
            flush: cons(),
            close: cons(),
        }
    }
    fn apply<U>(&self, f: impl Fn(&T) -> U) -> SinkStages<U> {
        SinkStages {
            ready: f(&self.ready),
            flush: f(&self.flush),
            close: f(&self.close),
        }
    }
}

pub struct IdChannel {
    out_channel: (
        Pin<Box<UnboundedReceiver<Item>>>,
        Pin<Box<UnboundedSender<Item>>>,
    ),
    set_waker: SinkStages<SetWaker<RawMutex, ForkHandle>>,
    context: Context,
    in_channels:
        Arc<Mutex<HashMap<ForkHandle, (Sink<Box<dyn SerdeAny>, ChannelError>, SinkStages<Waker>)>>>,
}

#[derive(Clone)]
struct IdChannelHandle {
    out_channel: Pin<Box<UnboundedSender<Item>>>,
    context: Context,
    set_waker: SinkStages<SetWaker<RawMutex, ForkHandle>>,
    in_channels:
        Arc<Mutex<HashMap<ForkHandle, (Sink<Box<dyn SerdeAny>, ChannelError>, SinkStages<Waker>)>>>,
}

impl IdChannelHandle {
    fn remove_fork(&self, handle: ForkHandle) {
        self.in_channels.lock().unwrap().remove(&handle);
    }
}

impl Stream for IdChannel {
    type Item = Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.out_channel.0.as_mut().poll_next(cx)
    }
}

#[derive(Debug)]
pub enum SinkStage {
    Ready,
    Send,
    Flush,
    Close,
}

impl Display for SinkStage {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                SinkStage::Ready => "ready",
                SinkStage::Send => "send",
                SinkStage::Flush => "flush",
                SinkStage::Close => "close",
            }
        )
    }
}

#[derive(Debug, Error)]
pub enum IdChannelError {
    #[error("send on underlying channel {1} in {0} stage failed: {2}")]
    Channel(SinkStage, ForkHandle, ChannelError),
    #[error("underlying channel {0} does not exist")]
    InvalidId(ForkHandle),
}

impl Drop for IdChannel {
    fn drop(&mut self) {
        self.in_channels.lock().unwrap().remove(&ForkHandle(0));
    }
}

impl ISink<Item> for IdChannel {
    type Error = IdChannelError;

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        match self.in_channels.lock().unwrap().get_mut(&item.0) {
            Some(channel) => {
                let (id, data) = (item.0, item.1);
                channel
                    .0
                    .as_mut()
                    .start_send(data)
                    .map_err(move |e| IdChannelError::Channel(SinkStage::Send, id, e))
            }
            None => Err(IdChannelError::InvalidId(item.0)),
        }
    }
    fn poll_ready(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.set_waker.ready.register(cx.waker());
        let mut in_channels = self.in_channels.lock().unwrap();
        for key in self.set_waker.ready.keys() {
            let channel = in_channels.get_mut(&key).unwrap();
            for key in self.set_waker.flush.keys() {
                if let Some(channel) = in_channels.get_mut(&key) {
                    let _ = channel
                        .0
                        .as_mut()
                        .poll_ready(&mut FContext::from_waker(&channel.1.ready))
                        .map_err(move |e| IdChannelError::Channel(SinkStage::Ready, key, e))?;
                }
            }
        }
        Poll::Ready(Ok(()))
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.set_waker.flush.register(cx.waker());
        let mut in_channels = self.in_channels.lock().unwrap();
        for key in self.set_waker.flush.keys() {
            if let Some(channel) = in_channels.get_mut(&key) {
                let _ = channel
                    .0
                    .as_mut()
                    .poll_ready(&mut FContext::from_waker(&channel.1.flush))
                    .map_err(move |e| IdChannelError::Channel(SinkStage::Flush, key, e))?;
            }
        }
        Poll::Ready(Ok(()))
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.set_waker.close.register(cx.waker());
        let mut in_channels = self.in_channels.lock().unwrap();
        for key in self.set_waker.close.keys() {
            let channel = in_channels.get_mut(&key).unwrap();
            for key in self.set_waker.flush.keys() {
                if let Some(channel) = in_channels.get_mut(&key) {
                    let _ = channel
                        .0
                        .as_mut()
                        .poll_ready(&mut FContext::from_waker(&channel.1.close))
                        .map_err(move |e| IdChannelError::Channel(SinkStage::Close, key, e))?;
                }
            }
        }
        Poll::Ready(Ok(()))
    }
}

impl Waiter for Context {
    fn wait_for(&self, data: String) -> Future<()> {
        Box::pin(self.wait_for(ForkHandle(data.parse().unwrap())))
    }
}

impl<'de> IContext<'de> for IdChannel {
    type Item = Item;
    type Target = Context;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

impl<'de> IContext<'de> for IdChannelHandle {
    type Item = Item;
    type Target = Context;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

pub struct Shim<K: Kind> {
    context: Context,
    _marker: PhantomData<K>,
}

impl<'a, K: Kind> IShim<'a, IdChannel, K> for Shim<K> {
    fn complete<C: Sync + Send + Stream<Item = Item> + ISink<Item> + 'static>(
        self,
        input: C,
    ) -> Fallible<K, K::ConstructError> {
        let (sink, stream) = input.split();
        let (sender, receiver) = unbounded();
        let channel = IdChannel {
            out_channel: (Box::pin(receiver), Box::pin(sender)),
            context: self.context,
            set_waker: SinkStages::new(|| SetWaker::new()),
            in_channels: Arc::new(Mutex::new(HashMap::new())),
        };
        let fork = channel.get_fork::<K>(ForkHandle(0));
        let (sender, receiver) = channel.split();
        spawn(receiver.map(Ok).forward(sink).unwrap_or_else(|_| panic!()));
        spawn(
            stream
                .map(Ok)
                .forward(sender)
                .unwrap_or_else(|e| panic!(format!("{}", e))),
        );
        Box::pin(fork)
    }
}

impl<'a, K: Kind> IContext<'a> for Shim<K> {
    type Item = Item;
    type Target = Context;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

impl IdChannelHandle {
    fn fork<K: Kind>(&self, kind: K) -> Fallible<ForkHandle, K::DeconstructError> {
        REGISTRY.add_deconstruct::<K>();
        let id = self.context.create::<K>();
        let waker = self.set_waker.apply(|waker| waker.with_key(id.clone()));
        let context = self.context.clone();
        let out_channel = self.out_channel.clone();
        let in_channels = self.in_channels.clone();

        Box::pin(
            IdChannelFork::new(kind, self.clone(), id).map(move |(sender, receiver)| {
                spawn(
                    receiver
                        .map(move |v| Ok(Item::new(id, Box::new(v), context.clone())))
                        .forward(out_channel)
                        .unwrap_or_else(|_| panic!()),
                );
                let mut in_channels = in_channels.lock().unwrap();
                in_channels.insert(
                    id,
                    (
                        Box::pin(
                            sender
                                .with(|item: Box<dyn SerdeAny>| {
                                    ok(*(item
                                        .downcast::<K::DeconstructItem>()
                                        .map_err(|e| panic!(e))
                                        .unwrap()))
                                })
                                .sink_map_err(|e: SendError| ChannelError(e.into())),
                        ),
                        waker,
                    ),
                );
                Ok(id)
            }),
        )
    }

    fn get_fork<K: Kind>(&self, fork_ref: ForkHandle) -> Fallible<K, K::ConstructError> {
        let out_channel = self.out_channel.clone();
        REGISTRY.add_construct::<K>();
        self.context.add::<K>(fork_ref);
        let (sender, ireceiver): (UnboundedSender<K::DeconstructItem>, _) = unbounded();
        let (isender, receiver): (UnboundedSender<K::ConstructItem>, _) = unbounded();
        let isender = isender.with(|item: Box<dyn SerdeAny>| {
            ok(*(match item.downcast::<K::ConstructItem>() {
                Ok(item) => item,
                Err(_) => panic!(),
            }))
        });
        self.in_channels.lock().unwrap().insert(
            fork_ref.clone(),
            (
                Box::pin(isender.sink_map_err(|e: SendError| ChannelError(e.into()))),
                self.set_waker
                    .apply(|waker| waker.with_key(fork_ref.clone())),
            ),
        );
        let ct = self.context.clone();
        spawn(
            ireceiver
                .map(move |item: K::DeconstructItem| {
                    Ok(Item::new(fork_ref, Box::new(item), ct.clone()))
                })
                .forward(out_channel)
                .unwrap_or_else(|_| panic!()),
        );
        Box::pin(K::construct(IdChannelFork {
            o: Box::pin(sender),
            i: Box::pin(receiver),
            handle: fork_ref,
            channel: self.clone(),
            sink_item: PhantomData,
        }))
    }
}

impl IdChannel {
    fn clone(&self) -> IdChannelHandle {
        IdChannelHandle {
            out_channel: self.out_channel.1.clone(),
            context: self.context.clone(),
            in_channels: self.in_channels.clone(),
            set_waker: self.set_waker.clone(),
        }
    }
    fn get_fork<K: Kind>(&self, fork_ref: ForkHandle) -> Fallible<K, K::ConstructError> {
        self.clone().get_fork(fork_ref)
    }
}

impl<'a, K: Kind> Target<'a, K> for IdChannel {
    type Shim = Shim<K>;

    fn new_with(kind: K) -> Future<Self>
    where
        K::DeconstructFuture: Send,
    {
        Box::pin(IdChannelFork::new_root(kind))
    }

    fn new_shim() -> Self::Shim {
        REGISTRY.add_construct::<K>();
        let context = Context::new_shim();
        context.add::<K>(ForkHandle(0));
        Shim {
            context,
            _marker: PhantomData,
        }
    }
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + 'static,
        O: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
    > IFork for IdChannelFork<I, O>
{
    fn fork<K: Kind>(&self, kind: K) -> Fallible<ForkHandle, K::DeconstructError> {
        self.channel.fork(kind)
    }
    fn get_fork<K: Kind>(&self, fork_ref: ForkHandle) -> Fallible<K, K::ConstructError> {
        self.channel.get_fork(fork_ref)
    }
}

pub(crate) struct IdChannelFork<
    I: Serialize + DeserializeOwned + Sync + Send + 'static,
    O: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
> {
    i: Pin<Box<UnboundedReceiver<I>>>,
    o: Pin<Box<UnboundedSender<O>>>,
    channel: IdChannelHandle,
    handle: ForkHandle,
    sink_item: PhantomData<O>,
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + 'static,
        O: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
    > Drop for IdChannelFork<I, O>
{
    fn drop(&mut self) {
        self.channel.remove_fork(self.handle);
    }
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + 'static,
        O: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
    > Stream for IdChannelFork<I, O>
{
    type Item = I;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.i.as_mut().poll_next(cx)
    }
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
        O: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
    > IdChannelFork<I, O>
{
    fn new<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
        channel: IdChannelHandle,
        handle: ForkHandle,
    ) -> impl IFuture<Output = (UnboundedSender<I>, UnboundedReceiver<O>)>
    where
        K::DeconstructFuture: Sync + Send + 'static,
    {
        async move {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            spawn(
                kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    handle,
                    channel,
                    sink_item: PhantomData,
                })
                .unwrap_or_else(|_| panic!()),
            );
            (sender, receiver)
        }
    }

    fn new_root<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
    ) -> impl IFuture<Output = IdChannel>
    where
        K::DeconstructFuture: Sync + Send + 'static,
    {
        async move {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            let mut in_channels = HashMap::new();
            REGISTRY.add_deconstruct::<K>();
            let context = Context::new();
            let handle = context.create::<K>();
            let set_waker = SinkStages::new(|| SetWaker::new());
            in_channels.insert(
                handle.clone(),
                (
                    Box::pin(
                        sender
                            .with(|item: Box<dyn SerdeAny>| {
                                ok(*(item
                                    .downcast::<K::DeconstructItem>()
                                    .map_err(|_| panic!())
                                    .unwrap()))
                            })
                            .sink_map_err(|e: SendError| ChannelError(e.into())),
                    ) as Sink<Box<dyn SerdeAny>, ChannelError>,
                    set_waker.apply(|waker| waker.with_key(handle.clone())),
                ),
            );
            let ct = context.clone();
            let (csender, creceiver) = unbounded();
            let channel = IdChannel {
                out_channel: (Box::pin(creceiver), Box::pin(csender.clone())),
                context,
                set_waker,
                in_channels: Arc::new(Mutex::new(in_channels)),
            };
            spawn(
                receiver
                    .map(move |v| Ok(Item::new(handle, Box::new(v), ct.clone())))
                    .forward(csender)
                    .unwrap_or_else(|_| panic!()),
            );
            spawn(
                kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    handle,
                    channel: channel.clone(),
                    sink_item: PhantomData,
                })
                .unwrap_or_else(|_| panic!()),
            );
            channel
        }
    }
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Sync + Send + 'static,
    > ISink<O> for IdChannelFork<I, O>
{
    type Error = ChannelError;

    fn start_send(mut self: Pin<&mut Self>, item: O) -> Result<(), Self::Error> {
        Ok(self
            .o
            .as_mut()
            .start_send(item)
            .map_err(|e| ChannelError(e.into()))?)
    }
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o
            .as_mut()
            .poll_ready(cx)
            .map_err(|e| ChannelError(e.into()))
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o
            .as_mut()
            .poll_flush(cx)
            .map_err(|e| ChannelError(e.into()))
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o
            .as_mut()
            .poll_close(cx)
            .map_err(|e| ChannelError(e.into()))
    }
}

impl<
        I: Serialize + DeserializeOwned + Sync + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Send + Sync + 'static,
    > Channel<I, O> for IdChannelFork<I, O>
{
}
