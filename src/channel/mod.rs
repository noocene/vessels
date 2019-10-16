pub mod id;
pub use id::IdChannel;

use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    Deserialize, Serialize,
};

use crate::Value;

use futures::{Future, Sink, Stream};

#[derive(Serialize, Deserialize)]
pub struct ForkHandle(u64);

pub trait Fork: Send + 'static {
    fn fork<V: Value>(&self, value: V) -> ForkHandle;
    fn get_fork<V: Value + Send + 'static>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = V, Error = ()> + Send + 'static>;
}

pub trait Channel<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
>: Stream<Item = I, Error = ()> + Sink<SinkItem = O, SinkError = ()> + Fork
{
    type Fork: Fork;

    fn split_factory(&self) -> Self::Fork;
}

pub trait Target:
    Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item>
{
    type Error;
    type Item;

    fn new_with<V: Value + Send + 'static>(
        value: V,
    ) -> Box<dyn Future<Item = Self, Error = <Self as Target>::Error> + Send + 'static>
    where
        V::DeconstructFuture: Send;

    fn new<
        V: Value + Send + 'static,
        C: Stream<Item = <Self as Target>::Item> + Sink<SinkItem = <Self as Target>::Item> + 'static,
    >(
        item: C,
    ) -> Box<dyn Future<Item = V, Error = <Self as Target>::Error> + Send + 'static>;
}

pub trait Context<'de> {
    type Item: Serialize + 'static;
    type Target: DeserializeSeed<'de, Value = Self::Item> + Clone + Send + 'static;

    fn context(&self) -> Self::Target;
}
