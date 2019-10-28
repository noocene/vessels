pub mod id;
pub use id::IdChannel;

use serde::{
    de::{DeserializeOwned, DeserializeSeed},
    Deserialize, Serialize,
};

use crate::Kind;

use futures::{Future, Sink, Stream};

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct ForkHandle(pub(crate) u32);

pub trait Fork: Send + 'static {
    fn fork<K: Kind>(
        &self,
        kind: K,
    ) -> Box<dyn Future<Item = ForkHandle, Error = ()> + Send + 'static>;
    fn get_fork<K: Kind>(
        &self,
        fork_ref: ForkHandle,
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static>;
}

pub trait Channel<
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
>: Stream<Item = I, Error = ()> + Sink<SinkItem = O, SinkError = ()> + Fork
{
    type Fork: Fork;

    fn split_factory(&self) -> Self::Fork;
}

pub trait Shim<'a, T: Target<'a, K>, K: Kind>:
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
    ) -> Box<dyn Future<Item = K, Error = ()> + Send + 'static>;
}

pub trait Target<'a, K: Kind>: Context<'a> + Sized {
    type Shim: Shim<'a, Self, K>;

    fn new_with(kind: K) -> Box<dyn Future<Item = Self, Error = ()> + Send + 'static>
    where
        K::DeconstructFuture: Send;

    fn new_shim() -> Self::Shim;
}

pub trait Context<'de> {
    type Item: Serialize + 'static;
    type Target: DeserializeSeed<'de, Value = Self::Item> + Clone + Send + 'static;

    fn context(&self) -> Self::Target;
}

pub trait OnTo: Kind {
    fn on_to<'a, T: Target<'a, Self>>(
        self,
    ) -> Box<dyn Future<Item = T, Error = ()> + Send + 'static>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send;
}

impl<K: Kind> OnTo for K {
    fn on_to<'a, T: Target<'a, Self>>(
        self,
    ) -> Box<dyn Future<Item = T, Error = ()> + Send + 'static>
    where
        Self: Send + 'static,
        Self::DeconstructFuture: Send,
    {
        T::new_with(self)
    }
}
