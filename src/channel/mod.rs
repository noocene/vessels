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

pub trait Shim<'a, T: Target<'a, V>, V: Value + Send + 'static>:
    Context<'a, Item = <T as Context<'a>>::Item>
{
    fn complete<
        C: Stream<Item = <T as Context<'a>>::Item>
            + Sink<SinkItem = <T as Context<'a>>::Item>
            + Send
            + 'static,
    >(
        self,
        input: C,
    ) -> Box<dyn Future<Item = V, Error = <T as Target<'a, V>>::Error> + Send + 'static>;
}

pub trait Target<'a, V: Value + Send + 'static>: Context<'a> + Sized {
    type Error: Send + 'static;
    type Shim: Shim<'a, Self, V>;

    fn new_with(
        value: V,
    ) -> Box<dyn Future<Item = Self, Error = <Self as Target<'a, V>>::Error> + Send + 'static>
    where
        V::DeconstructFuture: Send;

    fn new() -> Self::Shim;
}

pub trait Context<'de> {
    type Item: Serialize + 'static;
    type Target: DeserializeSeed<'de, Value = Self::Item> + Clone + Send + 'static;

    fn context(&self) -> Self::Target;
}

pub trait IntoStream: Value {
    fn into_stream<'a, T: Target<'a, Self>>(
        self,
    ) -> Box<dyn Future<Item = T, Error = <T as Target<'a, Self>>::Error> + Send + 'static>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send;
}

impl<V: Value> IntoStream for V {
    fn into_stream<'a, T: Target<'a, Self>>(
        self,
    ) -> Box<dyn Future<Item = T, Error = <T as Target<'a, Self>>::Error> + Send + 'static>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send,
    {
        T::new_with(self)
    }
}
