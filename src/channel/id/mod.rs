mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;
use id::REGISTRY;

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    executor::ThreadPool,
    future::{lazy, ok, BoxFuture},
    stream::{self, select, BoxStream},
    task::Context as FContext,
    Future, FutureExt, Poll, Sink, SinkExt, Stream, StreamExt, TryFutureExt,
};

use serde::{de::DeserializeOwned, Serialize};

use std::{collections::HashMap, ops::DerefMut};

use crate::{
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle},
    Kind, SerdeAny, Target,
};

use std::{
    marker::PhantomData,
    pin::Pin,
    sync::{Arc, Mutex},
};

use super::Shim as IShim;

pub struct IdChannel {
    out_channel: Arc<Mutex<BoxStream<'static, Item>>>,
    context: Context,
    in_channels:
        Arc<Mutex<HashMap<ForkHandle, Pin<Box<dyn Sink<Box<dyn SerdeAny>, Error = ()> + Send>>>>>,
}

impl Stream for IdChannel {
    type Item = Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.out_channel.lock().unwrap().as_mut().poll_next(cx)
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

impl<'de> IContext<'de> for IdChannel {
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
    fn complete<C: Stream<Item = Item> + Sink<Item> + Send + 'static>(
        self,
        input: C,
    ) -> BoxFuture<'static, Result<K, K::Error>> {
        let (sink, stream) = input.split();
        let channel = IdChannel {
            out_channel: Arc::new(Mutex::new(Box::pin(stream::empty()))),
            context: self.context,
            in_channels: Arc::new(Mutex::new(HashMap::new())),
        };
        let fork = channel.get_fork::<K>(ForkHandle(0));
        let (receiver, sender) = channel.split();
        let pool = ThreadPool::new().unwrap();
        pool.spawn_ok(
            sender
                .map(Ok::<_, <C as Sink<Item>>::Error>)
                .forward(sink)
                .unwrap_or_else(|_| panic!()),
        );
        pool.spawn_ok(
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

impl IdChannel {
    fn clone(&self) -> Self {
        IdChannel {
            out_channel: self.out_channel.clone(),
            context: self.context.clone(),
            in_channels: self.in_channels.clone(),
        }
    }
    fn fork<K: Kind>(&self, kind: K) -> BoxFuture<'static, ForkHandle> {
        let id = self.context.create::<K>();
        REGISTRY.add::<K::DeconstructItem>();
        let context = self.context.clone();
        let out_channel = self.out_channel.clone();
        let in_channels = self.in_channels.clone();

        Box::pin(
            IdChannelFork::<Box<UnboundedReceiver<_>>, Box<UnboundedSender<_>>, _, _>::new(
                kind, self,
            )
            .map(move |(sender, receiver)| {
                let mut out_channel = out_channel.lock().unwrap();
                let mut empty_stream = Box::pin(stream::empty()) as BoxStream<'static, Item>;
                std::mem::swap(&mut (*out_channel), &mut empty_stream);
                *out_channel = Box::pin(select(
                    empty_stream,
                    receiver.map(move |item| Item::new(id, Box::new(item), context.clone())),
                ));
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
                id
            }),
        )
    }

    fn get_fork<K: Kind>(&self, fork_ref: ForkHandle) -> BoxFuture<'static, Result<K, K::Error>> {
        let channel = self.clone();

        let mut in_channels = channel.in_channels.lock().unwrap();
        let mut out_channel = channel.out_channel.lock().unwrap();
        channel.context.add::<K>(fork_ref);
        REGISTRY.add::<K::ConstructItem>();
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
        in_channels.insert(fork_ref, Box::pin(isender));
        let ct = channel.context.clone();
        let ireceiver = ireceiver
            .map(move |item: K::DeconstructItem| Item::new(fork_ref, Box::new(item), ct.clone()));
        let mut empty_stream = Box::pin(stream::empty()) as BoxStream<'static, Item>;
        std::mem::swap(&mut (*out_channel), &mut empty_stream);
        *out_channel = Box::pin(select(empty_stream, ireceiver));
        Box::pin(K::construct(IdChannelFork {
            o: Box::pin(sender),
            i: Box::pin(receiver),
            channel: channel.clone(),
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
        Box::pin(IdChannelFork::<
            Box<UnboundedReceiver<_>>,
            Box<UnboundedSender<_>>,
            _,
            _,
        >::new_root(kind))
    }

    fn new_shim() -> Self::Shim {
        Shim {
            context: Context::new_with::<K>(),
            _marker: PhantomData,
        }
    }
}

impl<
        T: Send + Unpin + 'static,
        U: Send + Unpin + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + Unpin + 'static,
    > IFork for IdChannelFork<T, U, I, O>
where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
    fn fork<K: Kind>(&self, kind: K) -> BoxFuture<'static, ForkHandle> {
        self.channel.fork(kind)
    }
    fn get_fork<K: Kind>(&self, fork_ref: ForkHandle) -> BoxFuture<'static, Result<K, K::Error>> {
        self.channel.get_fork(fork_ref)
    }
}

pub(crate) struct IdChannelFork<
    T: Send + Unpin + 'static,
    U: Send + Unpin + 'static,
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + Unpin + 'static,
> where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
    i: Pin<T>,
    o: Pin<U>,
    channel: IdChannel,
    sink_item: PhantomData<O>,
}

impl<
        T: Send + Unpin + 'static,
        U: Send + Unpin + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + Unpin + 'static,
    > Stream for IdChannelFork<T, U, I, O>
where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
    type Item = I;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.i.as_mut().poll_next(cx)
    }
}

impl<
        T: Send + Unpin + 'static,
        U: Send + Unpin + 'static,
        I: Serialize + DeserializeOwned + Send + Unpin + 'static,
        O: Serialize + DeserializeOwned + Send + Unpin + 'static,
    > IdChannelFork<T, U, I, O>
where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
    fn new<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
        channel: &'_ IdChannel,
    ) -> impl Future<Output = (UnboundedSender<I>, UnboundedReceiver<O>)>
    where
        K::DeconstructFuture: Send + 'static,
    {
        let channel = channel.clone();
        lazy(move |_| {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            ThreadPool::new()
                .unwrap()
                .spawn_ok(kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    channel,
                    sink_item: PhantomData,
                }));
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
            REGISTRY.add::<K::ConstructItem>();
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
            let context = Context::new_with::<K>();
            let ct = context.clone();
            let channel = IdChannel {
                out_channel: Arc::new(Mutex::new(Box::pin(
                    receiver.map(move |v| Item::new(ForkHandle(0), Box::new(v), ct.clone())),
                ))),
                context,
                in_channels: Arc::new(Mutex::new(in_channels)),
            };
            ThreadPool::new()
                .unwrap()
                .spawn_ok(kind.deconstruct(IdChannelFork {
                    o: Box::pin(oi),
                    i: Box::pin(oo),
                    channel: channel.clone(),
                    sink_item: PhantomData,
                }));
            channel
        })
    }
}

impl<
        T: Send + Unpin + 'static,
        U: Send + Unpin + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Send + 'static,
    > Sink<O> for IdChannelFork<T, U, I, O>
where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
    type Error = <U::Target as Sink<O>>::Error;

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
        T: Send + Unpin + 'static,
        U: Send + Unpin + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + Unpin + DeserializeOwned + Send + 'static,
    > Channel<I, O> for IdChannelFork<T, U, I, O>
where
    T: DerefMut,
    U: DerefMut,
    U::Target: Sink<O>,
    T::Target: Stream<Item = I>,
    <U::Target as Sink<O>>::Error: Send + 'static,
{
}
