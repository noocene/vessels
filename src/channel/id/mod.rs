mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;

use futures::{
    future::empty,
    lazy, stream,
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};

use serde::{de::DeserializeOwned, Serialize};

use std::collections::HashMap;

use crate::{
    channel::Fork,
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle},
    format::StreamSink,
    SerdeAny, Target, Value,
};

use std::{
    any::{Any, TypeId},
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
        Ok(Async::Ready(()))
    }
}

impl<'de> IContext<'de> for IdChannel {
    type Item = Item;
    type Target = Context;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

pub struct Shim<V: Value> {
    context: Context,
    _marker: PhantomData<V>,
}

struct TestShim<T: Stream, E: Sink> {
    m: StreamSink<T, E>,
}

impl<T: Stream, U: Sink> Sink for TestShim<T, U> {
    type SinkItem = U::SinkItem;
    type SinkError = U::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.m.start_send(item)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.m.poll_complete()
    }
}

impl<T: Stream, U: Sink> Stream for TestShim<T, U> {
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.m.poll()
    }
}

impl<T, U> Channel<T::Item, U::SinkItem> for TestShim<T, U>
where
    U: Sink<SinkError = ()> + Send + 'static,
    U::SinkItem: Serialize + DeserializeOwned + Send + 'static,
    T: Stream<Error = ()> + Send + 'static,
    T::Item: Serialize + DeserializeOwned + Send + 'static,
{
    type Fork = Self;

    fn split_factory(&self) -> Self::Fork {
        panic!()
    }
}

impl<T: Stream + Send + 'static, U: Sink + Send + 'static> Fork for TestShim<T, U> {
    fn fork<V: Value>(&self, value: V) -> ForkHandle {
        ForkHandle(0)
    }
    fn get_fork<V: Value + Send + 'static>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = V, Error = ()> + Send + 'static> {
        Box::new(empty())
    }
}

impl<'a, V: Value + Send + 'static> IShim<'a, IdChannel, V> for Shim<V> {
    fn complete<C: Stream<Item = Item> + Sink<SinkItem = Item> + Send + 'static>(
        self,
        input: C,
    ) -> Box<dyn Future<Item = V, Error = <IdChannel as Target<'a, V>>::Error> + Send + 'static>
    {
        Box::new(lazy(|| {
            let channel = IdChannel::new();
            let (sink, stream) = input.split();
            let sink = sink
                .sink_map_err(|_: <C as Sink>::SinkError| ())
                .with(move |v: V::DeconstructItem| {
                    Ok(Item::new(0, Box::new(v) as Box<dyn SerdeAny>))
                })
                .sink_map_err(|_: ()| ());
            let stream = stream
                .map(|item| {
                    *(item
                        .1
                        .downcast::<V::ConstructItem>()
                        .map_err(|_| ())
                        .unwrap())
                })
                .map_err(|_| ());
            V::construct(TestShim {
                m: StreamSink(stream, sink),
            })
            .map_err(|_| ())
        }))
    }
}

impl<'a, V: Value> IContext<'a> for Shim<V> {
    type Item = Item;
    type Target = Context;

    fn context(&self) -> Self::Target {
        self.context.clone()
    }
}

impl IdChannel {
    fn new() -> Self {
        IdChannel {
            out_channel: Arc::new(Mutex::new(Box::new(stream::empty()))),
            context: Context::new(),
            in_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn clone(&self) -> Self {
        IdChannel {
            out_channel: self.out_channel.clone(),
            context: self.context.clone(),
            in_channels: self.in_channels.clone(),
        }
    }
}

impl<'a, V: Value + Send + 'static> Target<'a, V> for IdChannel {
    type Error = ();
    type Shim = Shim<V>;

    fn new_with(
        value: V,
    ) -> Box<dyn Future<Item = Self, Error = <Self as Target<'a, V>>::Error> + Send + 'static>
    where
        V::DeconstructFuture: Send,
    {
        Box::new(IdChannelFork::new_with(value))
    }

    fn new_shim() -> Self::Shim {
        Shim {
            context: Context::new_with::<V>(),
            _marker: PhantomData,
        }
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IFork for IdChannelFork<I, O>
{
    fn fork<V: Value>(&self, value: V) -> ForkHandle {
        ForkHandle(0)
    }
    fn get_fork<V: Value + Send + 'static>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = V, Error = ()> + Send + 'static> {
        Box::new(empty())
    }
}

pub(crate) struct IdChannelFork<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
> {
    i: UnboundedReceiver<I>,
    o: UnboundedSender<O>,
    channel: IdChannel,
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Stream for IdChannelFork<I, O>
{
    type Item = I;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.i.poll().map_err(|e| ())
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IdChannelFork<I, O>
{
    fn new_with<V: Value<DeconstructItem = I, ConstructItem = O>>(
        value: V,
    ) -> impl Future<Item = IdChannel, Error = ()>
    where
        V::DeconstructFuture: Send + 'static,
    {
        lazy(move || {
            let (sender, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, receiver): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            let mut in_channels = HashMap::new();
            in_channels.insert(
                0u32,
                Box::new(sender.sink_map_err(|_| ()).with(|item: Box<dyn SerdeAny>| {
                    Ok(*(item
                        .downcast::<V::DeconstructItem>()
                        .map_err(|_| ())
                        .unwrap()))
                }))
                    as Box<dyn Sink<SinkItem = Box<dyn SerdeAny>, SinkError = ()> + Send>,
            );
            let channel = IdChannel {
                out_channel: Arc::new(Mutex::new(Box::new(
                    receiver.map(move |v| Item::new(0, Box::new(v) as Box<dyn SerdeAny>)),
                ))),
                context: Context::new_with::<V>(),
                in_channels: Arc::new(Mutex::new(in_channels)),
            };
            tokio::spawn(
                value
                    .deconstruct(IdChannelFork {
                        o: oi,
                        i: oo,
                        channel: channel.clone(),
                    })
                    .map_err(|e| ()),
            );
            Ok(channel)
        })
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Sink for IdChannelFork<I, O>
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

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Channel<I, O> for IdChannelFork<I, O>
{
    type Fork = IdChannelFork<I, O>;

    fn split_factory(&self) -> Self::Fork {
        panic!()
    }
}
