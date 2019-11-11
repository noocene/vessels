mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;
use id::REGISTRY;

use futures::{
    channel::mpsc::{unbounded, SendError, UnboundedReceiver, UnboundedSender},
    future::{lazy, ok, BoxFuture},
    task::Context as FContext,
    Future, FutureExt, Poll, Sink, SinkExt, Stream, StreamExt, TryFutureExt,
};

use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;

use crate::{
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle, Waiter},
    core,
    core::{executor::Spawn, Executor},
    Kind, SerdeAny, Target,
};

use std::{
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Mutex},
};

use super::Shim as IShim;

pub struct IdChannel {
    out_channel: (
        Pin<Box<UnboundedReceiver<Item>>>,
        Pin<Box<UnboundedSender<Item>>>,
    ),
    context: Context,
    in_channels:
        Arc<Mutex<HashMap<ForkHandle, Pin<Box<dyn Sink<Box<dyn SerdeAny>, Error = ()> + Send>>>>>,
}

#[derive(Clone)]
struct IdChannelHandle {
    out_channel: Pin<Box<UnboundedSender<Item>>>,
    context: Context,
    in_channels:
        Arc<Mutex<HashMap<ForkHandle, Pin<Box<dyn Sink<Box<dyn SerdeAny>, Error = ()> + Send>>>>>,
}

impl Stream for IdChannel {
    type Item = Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.out_channel.0.as_mut().poll_next(cx)
    }
}

impl Sink<Item> for IdChannel {
    type Error = ();

    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        if let Some(channel) = self.in_channels.lock().unwrap().get_mut(&item.0) {
            channel.as_mut().start_send(item.1)
        } else {
            Err(())
        }
    }
    fn poll_ready(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        if let Some(result) = self
            .in_channels
            .lock()
            .unwrap()
            .values_mut()
            .map(|item| item.as_mut().poll_ready(cx))
            .find(|poll| match poll {
                Poll::Ready(_) => false,
                _ => true,
            })
        {
            result
        } else {
            Poll::Ready(Ok(()))
        }
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        if let Some(result) = self
            .in_channels
            .lock()
            .unwrap()
            .values_mut()
            .map(|item| item.as_mut().poll_flush(cx))
            .find(|poll| match poll {
                Poll::Ready(_) => false,
                _ => true,
            })
        {
            result
        } else {
            Poll::Ready(Ok(()))
        }
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        if let Some(result) = self
            .in_channels
            .lock()
            .unwrap()
            .values_mut()
            .map(|item| item.as_mut().poll_close(cx))
            .find(|poll| match poll {
                Poll::Ready(_) => false,
                _ => true,
            })
        {
            result
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

impl Waiter for Context {
    fn wait_for(&self, data: String) -> BoxFuture<'static, ()> {
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
    fn complete<C: Send + Stream<Item = Item> + Sink<Item> + 'static>(
        self,
        input: C,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>> {
        let (sink, stream) = input.split();
        let (sender, receiver) = unbounded();
        let channel = IdChannel {
            out_channel: (Box::pin(receiver), Box::pin(sender)),
            context: self.context,
            in_channels: Arc::new(Mutex::new(HashMap::new())),
        };
        let fork = channel.get_fork::<K>(ForkHandle(0));
        let (receiver, sender) = channel.split();
        let mut executor = core::<dyn Executor>().unwrap();
        executor.spawn(sender.map(Ok).forward(sink).unwrap_or_else(|_| panic!()));
        executor.spawn(
            stream
                .map(Ok)
                .forward(receiver)
                .unwrap_or_else(|_| panic!()),
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
    fn fork<K: Kind>(
        &self,
        kind: K,
    ) -> BoxFuture<'static, Result<ForkHandle, K::DeconstructError>> {
        REGISTRY.add_deconstruct::<K>();
        let id = self.context.create::<K>();
        let context = self.context.clone();
        let out_channel = self.out_channel.clone();
        let in_channels = self.in_channels.clone();

        Box::pin(
            IdChannelFork::new(kind, self.clone()).map(move |(sender, receiver)| {
                core::<dyn Executor>().unwrap().spawn(
                    receiver
                        .map(move |v| Ok(Item::new(id, Box::new(v), context.clone())))
                        .forward(out_channel)
                        .unwrap_or_else(|_| panic!()),
                );
                let mut in_channels = in_channels.lock().unwrap();
                in_channels.insert(
                    id,
                    Box::pin(
                        sender
                            .sink_map_err(|e| panic!(e))
                            .with(|item: Box<dyn SerdeAny>| {
                                ok(*(item
                                    .downcast::<K::DeconstructItem>()
                                    .map_err(|e| panic!(e))
                                    .unwrap()))
                            }),
                    ),
                );
                Ok(id)
            }),
        )
    }

    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>> {
        let out_channel = self.out_channel.clone();
        REGISTRY.add_construct::<K>();
        self.context.add::<K>(fork_ref);
        let (sender, ireceiver): (UnboundedSender<K::DeconstructItem>, _) = unbounded();
        let (isender, receiver): (UnboundedSender<K::ConstructItem>, _) = unbounded();
        let isender = isender
            .sink_map_err(|e| panic!(e))
            .with(|item: Box<dyn SerdeAny>| {
                ok(*(match item.downcast::<K::ConstructItem>() {
                    Ok(item) => item,
                    Err(_) => panic!(),
                }))
            });
        self.in_channels
            .lock()
            .unwrap()
            .insert(fork_ref, Box::pin(isender));
        let ct = self.context.clone();
        let ireceiver = ireceiver
            .map(move |item: K::DeconstructItem| Item::new(fork_ref, Box::new(item), ct.clone()));
        core::<dyn Executor>().unwrap().spawn(
            ireceiver
                .map(Ok)
                .forward(out_channel)
                .unwrap_or_else(|_| panic!()),
        );
        Box::pin(K::construct(IdChannelFork {
            o: Box::pin(sender),
            i: Box::pin(receiver),
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
        }
    }
    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>> {
        let out_channel = self.out_channel.1.clone();
        REGISTRY.add_construct::<K>();
        self.context.add::<K>(fork_ref);
        let (sender, ireceiver): (UnboundedSender<K::DeconstructItem>, _) = unbounded();
        let (isender, receiver): (UnboundedSender<K::ConstructItem>, _) = unbounded();
        let isender = isender
            .sink_map_err(|e| panic!(e))
            .with(|item: Box<dyn SerdeAny>| {
                ok(*(match item.downcast::<K::ConstructItem>() {
                    Ok(item) => item,
                    Err(_) => panic!(),
                }))
            });
        self.in_channels
            .lock()
            .unwrap()
            .insert(fork_ref, Box::pin(isender));
        let ct = self.context.clone();
        let ireceiver = ireceiver
            .map(move |item: K::DeconstructItem| Item::new(fork_ref, Box::new(item), ct.clone()));
        core::<dyn Executor>().unwrap().spawn(
            ireceiver
                .map(Ok)
                .forward(out_channel)
                .unwrap_or_else(|_| panic!()),
        );
        Box::pin(K::construct(IdChannelFork {
            o: Box::pin(sender),
            i: Box::pin(receiver),
            channel: self.clone(),
            sink_item: PhantomData,
        }))
    }
}

impl<'a, K: Kind> Target<'a, K> for IdChannel {
    type Shim = Shim<K>;

    fn new_with(kind: K) -> BoxFuture<'static, Self>
    where
        K::DeconstructFuture: Send,
    {
        Box::pin(IdChannelFork::new_root(kind))
    }

    fn new_shim() -> Self::Shim {
        REGISTRY.add_construct::<K>();
        let context = Context::new();
        context.add::<K>(ForkHandle(0));
        Shim {
            context,
            _marker: PhantomData,
        }
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + Unpin + 'static,
    > IFork for IdChannelFork<I, O>
{
    fn fork<K: Kind>(
        &self,
        kind: K,
    ) -> BoxFuture<'static, Result<ForkHandle, K::DeconstructError>> {
        self.channel.fork(kind)
    }
    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> BoxFuture<'static, Result<K, K::ConstructError>> {
        self.channel.get_fork(fork_ref)
    }
}

pub(crate) struct IdChannelFork<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + Unpin + 'static,
> {
    i: Pin<Box<UnboundedReceiver<I>>>,
    o: Pin<Box<UnboundedSender<O>>>,
    channel: IdChannelHandle,
    sink_item: PhantomData<O>,
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + Unpin + 'static,
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
    ) -> impl Future<Output = (UnboundedSender<I>, UnboundedReceiver<O>)>
    where
        K::DeconstructFuture: Send + 'static,
    {
        lazy(move |_| {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            core::<dyn Executor>().unwrap().spawn(
                kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    channel,
                    sink_item: PhantomData,
                })
                .unwrap_or_else(|_| panic!()),
            );
            (sender, receiver)
        })
    }

    fn new_root<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
    ) -> impl Future<Output = IdChannel>
    where
        K::DeconstructFuture: Send + 'static,
    {
        lazy(move |_| {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            let mut in_channels = HashMap::new();
            REGISTRY.add_deconstruct::<K>();
            in_channels.insert(
                ForkHandle(0),
                Box::pin(
                    sender
                        .sink_map_err(|e| panic!(e))
                        .with(|item: Box<dyn SerdeAny>| {
                            ok(*(item
                                .downcast::<K::DeconstructItem>()
                                .map_err(|_| panic!())
                                .unwrap()))
                        }),
                ) as Pin<Box<dyn Sink<Box<dyn SerdeAny>, Error = ()> + Send>>,
            );
            let context = Context::new();
            let handle = context.create::<K>();
            let ct = context.clone();
            let (csender, creceiver) = unbounded();
            let channel = IdChannel {
                out_channel: (Box::pin(creceiver), Box::pin(csender.clone())),
                context,
                in_channels: Arc::new(Mutex::new(in_channels)),
            };
            let mut executor = core::<dyn Executor>().unwrap();
            executor.spawn(
                receiver
                    .map(move |v| Ok(Item::new(handle, Box::new(v), ct.clone())))
                    .forward(csender)
                    .unwrap_or_else(|_| panic!()),
            );
            executor.spawn(
                kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    channel: channel.clone(),
                    sink_item: PhantomData,
                })
                .unwrap_or_else(|_| panic!()),
            );
            channel
        })
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Send + 'static,
    > Sink<O> for IdChannelFork<I, O>
{
    type Error = SendError;

    fn start_send(mut self: Pin<&mut Self>, item: O) -> Result<(), Self::Error> {
        self.o.as_mut().start_send(item)
    }
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o.as_mut().poll_ready(cx)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o.as_mut().poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.o.as_mut().poll_close(cx)
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Send + Sync + 'static,
    > Channel<I, O> for IdChannelFork<I, O>
{
}
