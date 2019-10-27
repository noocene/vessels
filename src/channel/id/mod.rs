mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;
use id::REGISTRY;

use futures::{
    lazy, stream,
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};

use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;

use crate::{
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle},
    Kind, SerdeAny, Target,
};

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use super::Shim as IShim;

pub struct IdChannel {
    out_channel: Arc<Mutex<Box<dyn Stream<Item = Item, Error = ()> + Send>>>,
    context: Context,
    in_channels: Arc<
        Mutex<HashMap<u32, Box<dyn Sink<SinkItem = Box<dyn SerdeAny>, SinkError = ()> + Send>>>,
    >,
}

impl Stream for IdChannel {
    type Item = Item;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.out_channel.lock().unwrap().poll()
    }
}

impl Sink for IdChannel {
    type SinkItem = Item;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        if let Some(channel) = self.in_channels.lock().unwrap().get_mut(&item.0) {
            channel.start_send(item.1).map(|a| {
                if let AsyncSink::Ready = a {
                    AsyncSink::Ready
                } else {
                    panic!()
                }
            })
        } else {
            Err(())
        }
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        if let Some(result) = self
            .in_channels
            .lock()
            .unwrap()
            .values_mut()
            .map(|item| item.poll_complete())
            .find(|poll| match poll {
                Ok(Async::Ready(())) => false,
                _ => true,
            })
        {
            result
        } else {
            Ok(Async::Ready(()))
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
    fn complete<C: Stream<Item = Item> + Sink<SinkItem = Item> + Send + 'static>(
        self,
        input: C,
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static> {
        Box::new(lazy(|| {
            let (sink, stream) = input.split();
            let channel = IdChannel {
                out_channel: Arc::new(Mutex::new(Box::new(stream::empty()))),
                context: self.context,
                in_channels: Arc::new(Mutex::new(HashMap::new())),
            };
            let fork = channel.get_fork::<K>(ForkHandle(0));
            let (receiver, sender) = channel.split();
            tokio::spawn(
                sender
                    .map_err(|_| panic!())
                    .forward(sink.sink_map_err(|_| panic!()))
                    .map_err(|_| panic!())
                    .and_then(|_| Ok(())),
            );
            tokio::spawn(
                stream
                    .map_err(|_| panic!())
                    .forward(receiver)
                    .map_err(|_| panic!())
                    .and_then(|_| Ok(())),
            );
            fork
        }))
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
    fn fork<K: Kind>(&self, kind: K) -> Box<dyn Future<Item = ForkHandle, Error = ()> + Send> {
        let id = self.context.create::<K>();
        REGISTRY.add::<K::DeconstructItem>();
        let context = self.context.clone();
        let out_channel = self.out_channel.clone();
        let in_channels = self.in_channels.clone();

        Box::new(
            IdChannelFork::<UnboundedReceiver<_>, UnboundedSender<_>, _, _>::new(kind, self)
                .and_then(move |(sender, receiver)| {
                    let mut out_channel = out_channel.lock().unwrap();
                    let mut empty_stream = Box::new(stream::empty())
                        as Box<dyn Stream<Item = Item, Error = ()> + Send>;
                    std::mem::swap(&mut (*out_channel), &mut empty_stream);
                    *out_channel = Box::new(empty_stream.select(
                        receiver.map(move |item| Item::new(id, Box::new(item), context.clone())),
                    ));
                    let mut in_channels = in_channels.lock().unwrap();
                    in_channels.insert(
                        id,
                        Box::new(sender.sink_map_err(|_| panic!()).with(
                            |item: Box<dyn SerdeAny>| {
                                Ok(*(item
                                    .downcast::<K::DeconstructItem>()
                                    .map_err(|_| panic!())
                                    .unwrap()))
                            },
                        )),
                    );
                    Ok(ForkHandle(id))
                }),
        )
    }

    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static> {
        let channel = self.clone();
        Box::new(lazy(move || {
            let mut in_channels = channel.in_channels.lock().unwrap();
            let mut out_channel = channel.out_channel.lock().unwrap();
            channel.context.add::<K>(fork_ref.0);
            REGISTRY.add::<K::ConstructItem>();
            let (sender, ireceiver): (UnboundedSender<K::DeconstructItem>, _) = unbounded();
            let (isender, receiver): (UnboundedSender<K::ConstructItem>, _) = unbounded();
            let isender = isender
                .sink_map_err(|_: <UnboundedSender<K::ConstructItem> as Sink>::SinkError| panic!())
                .with(|item: Box<dyn SerdeAny>| {
                    Ok(*(item
                        .downcast::<K::ConstructItem>()
                        .map_err(|_| panic!())
                        .unwrap()))
                })
                .sink_map_err(|_: ()| panic!());
            in_channels.insert(fork_ref.0, Box::new(isender));
            let ct = channel.context.clone();
            let ireceiver = ireceiver
                .map(move |item: K::DeconstructItem| {
                    Item::new(fork_ref.0, Box::new(item), ct.clone())
                })
                .map_err(|_| panic!());
            let mut empty_stream =
                Box::new(stream::empty()) as Box<dyn Stream<Item = Item, Error = ()> + Send>;
            std::mem::swap(&mut (*out_channel), &mut empty_stream);
            *out_channel = Box::new(empty_stream.select(ireceiver));
            K::construct(IdChannelFork {
                o: sender,
                i: receiver,
                channel: channel.clone(),
            })
            .map_err(|_| panic!())
        }))
    }
}

impl<'a, K: Kind> Target<'a, K> for IdChannel {
    type Shim = Shim<K>;

    fn new_with(kind: K) -> Box<dyn Future<Item = Self, Error = ()> + Send + 'static>
    where
        K::DeconstructFuture: Send,
    {
        Box::new(IdChannelFork::<
            UnboundedReceiver<_>,
            UnboundedSender<_>,
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
        T: Stream<Item = I> + Send + 'static,
        U: Sink<SinkItem = O> + Send + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IFork for IdChannelFork<T, U, I, O>
where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    fn fork<K: Kind>(&self, kind: K) -> Box<dyn Future<Item = ForkHandle, Error = ()> + Send> {
        self.channel.fork(kind)
    }
    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static> {
        self.channel.get_fork(fork_ref)
    }
}

pub(crate) struct IdChannelFork<
    T: Stream<Item = I> + Send + 'static,
    U: Sink<SinkItem = O> + Send + 'static,
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
> where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    i: T,
    o: U,
    channel: IdChannel,
}

impl<
        T: Stream<Item = I> + Send + 'static,
        U: Sink<SinkItem = O> + Send + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Stream for IdChannelFork<T, U, I, O>
where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    type Item = I;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.i.poll().map_err(|_| panic!())
    }
}

impl<
        T: Stream<Item = I> + Send + 'static,
        U: Sink<SinkItem = O> + Send + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IdChannelFork<T, U, I, O>
where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    fn new<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
        channel: &'_ IdChannel,
    ) -> impl Future<Item = (UnboundedSender<I>, UnboundedReceiver<O>), Error = ()>
    where
        K::DeconstructFuture: Send + 'static,
    {
        let channel = channel.clone();
        lazy(move || {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            tokio::spawn(
                kind.deconstruct(IdChannelFork {
                    o: oi,
                    i: oo,
                    channel,
                })
                .map_err(|_| panic!()),
            );
            Ok((sender, receiver))
        })
    }

    fn new_root<K: Kind<DeconstructItem = I, ConstructItem = O>>(
        kind: K,
    ) -> impl Future<Item = IdChannel, Error = ()>
    where
        K::DeconstructFuture: Send + 'static,
    {
        lazy(move || {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            let mut in_channels = HashMap::new();
            REGISTRY.add::<K::ConstructItem>();
            in_channels.insert(
                0u32,
                Box::new(
                    sender
                        .sink_map_err(|_| panic!())
                        .with(|item: Box<dyn SerdeAny>| {
                            Ok(*(item
                                .downcast::<K::DeconstructItem>()
                                .map_err(|_| panic!())
                                .unwrap()))
                        }),
                )
                    as Box<dyn Sink<SinkItem = Box<dyn SerdeAny>, SinkError = ()> + Send>,
            );
            let context = Context::new_with::<K>();
            let ct = context.clone();
            let channel = IdChannel {
                out_channel: Arc::new(Mutex::new(Box::new(
                    receiver.map(move |v| Item::new(0, Box::new(v), ct.clone())),
                ))),
                context,
                in_channels: Arc::new(Mutex::new(in_channels)),
            };
            tokio::spawn(
                kind.deconstruct(IdChannelFork {
                    o: oi,
                    i: oo,
                    channel: channel.clone(),
                })
                .map_err(|_| panic!()),
            );
            Ok(channel)
        })
    }
}

impl<
        T: Stream<Item = I> + Send + 'static,
        U: Sink<SinkItem = O> + Send + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Sink for IdChannelFork<T, U, I, O>
where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    type SinkItem = O;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.o.start_send(item).map_err(|e| panic!(e))
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.o.poll_complete().map_err(|e| panic!(e))
    }
}

impl IFork for IdFork {
    fn fork<K: Kind>(&self, kind: K) -> Box<dyn Future<Item = ForkHandle, Error = ()> + Send> {
        self.channel.fork(kind)
    }

    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static> {
        self.channel.get_fork(fork_ref)
    }
}
pub struct IdFork {
    channel: IdChannel,
}

impl<
        T: Stream<Item = I> + Send + 'static,
        U: Sink<SinkItem = O> + Send + 'static,
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Channel<I, O> for IdChannelFork<T, U, I, O>
where
    T::Error: Send + 'static,
    U::SinkError: Send + 'static,
{
    type Fork = IdFork;

    fn split_factory(&self) -> Self::Fork {
        IdFork {
            channel: self.channel.clone(),
        }
    }
}
