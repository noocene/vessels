mod context;
pub(crate) use context::Context;
mod item;
pub use item::Item;
mod id;
pub(crate) use id::Id;

use futures::{
    future::empty,
    lazy,
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    Future, Poll, Sink, StartSend, Stream,
};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    channel::Fork,
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle},
    format::StreamSink,
    SerdeAny, Target, Value,
};

use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use super::Shim as IShim;

pub struct IdChannel {
    out_channel: Box<dyn Stream<Item = Item, Error = ()> + Sync + Send>,
    context: Context,
}

impl Stream for IdChannel {
    type Item = Item;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.out_channel.poll()
    }
}

impl Sink for IdChannel {
    type SinkItem = Item;
    type SinkError = ();

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Err(())
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Err(())
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

impl<'a, V: Value + Send + 'static> Target<'a, V> for IdChannel {
    type Error = ();
    type Shim = Shim<V>;

    fn new_with(
        value: V,
    ) -> Box<dyn Future<Item = Self, Error = <Self as Target<'a, V>>::Error> + Send + 'static>
    where
        V::DeconstructFuture: Send,
    {
        Box::new(
            IdChannelFork::new_with(value).and_then(|(sender, receiver)| {
                Ok(IdChannel {
                    out_channel: Box::new(
                        receiver.map(move |v| Item::new(0, Box::new(v) as Box<dyn SerdeAny>)),
                    ),
                    context: Context::new::<V>(),
                })
            }),
        )
    }

    fn new() -> Self::Shim {
        Shim {
            context: Context::new::<V>(),
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
    ) -> impl Future<Item = (UnboundedSender<I>, UnboundedReceiver<O>), Error = ()>
    where
        V::DeconstructFuture: Send + 'static,
    {
        let (o, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
        let (oi, i): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
        lazy(move || {
            tokio::spawn(
                value
                    .deconstruct(IdChannelFork { o: oi, i: oo })
                    .map_err(|e| ()),
            );
            Ok((o, i))
        })
    }

    fn construct<
        V: Value<DeconstructItem = I, ConstructItem = O>,
        C: Stream<Item = Item> + Sink<SinkItem = Item>,
    >(
        input: C,
    ) -> impl Future<Item = V, Error = ()> {
        lazy(move || {
            let (o, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
            let (oi, i): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
            let (sender, receiver) = input.split();
            //V::construct(StreamSink(oo, o));
            Err(())
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
