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
    channel::{Channel, Context as IContext, Fork as IFork, ForkHandle},
    SerdeAny, Target, Value,
};

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

impl Target for IdChannel {
    type Error = ();
    type Item = Item;

    fn new_with<V: Value>(
        value: V,
    ) -> Box<dyn Future<Item = Self, Error = <Self as Target>::Error> + Send + 'static> {
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

    fn new<
        V: Value,
        C: Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item> + 'static,
    >(
        input: C,
    ) -> Box<dyn Future<Item = V, Error = <Self as Target>::Error> + Send + 'static> {
        Box::new(IdChannelFork::deconstruct(input))
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
    fn get_fork<V: Value>(
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
        self.i.poll().map_err(|_| ())
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > IdChannelFork<I, O>
{
    fn new_with<V: Value<DeconstructItem = I, ConstructItem = O>>(
        value: V,
    ) -> impl Future<Item = (UnboundedSender<I>, UnboundedReceiver<O>), Error = ()> {
        let (o, oo): (UnboundedSender<I>, UnboundedReceiver<I>) = unbounded();
        let (oi, i): (UnboundedSender<O>, UnboundedReceiver<O>) = unbounded();
        lazy(move || {
            tokio::spawn(value.deconstruct(IdChannelFork { o: oi, i: oo }));
            Ok((o, i))
        })
    }

    fn deconstruct<
        V: Value<DeconstructItem = I, ConstructItem = O>,
        C: Stream<Item = Item> + Sink<SinkItem = Item>,
    >(
        input: C,
    ) -> impl Future<Item = V, Error = ()> {
        lazy(move || {
            let _ = ();
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
        self.o.start_send(item).map_err(|_| ())
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.o.poll_complete().map_err(|_| ())
    }
}

impl<
        I: Serialize + DeserializeOwned + Send + 'static,
        O: Serialize + DeserializeOwned + Send + 'static,
    > Channel<I, O> for IdChannelFork<I, O>
{
    type ForkFactory = IdChannelFork<I, O>;

    fn split_factory(&self) -> Self::ForkFactory {
        panic!()
    }
}
