#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate mopa;

pub mod channel;
pub use channel::OnTo;
use channel::{Channel, Target};
pub mod format;
pub mod kind;

use erased_serde::Serialize as ErasedSerialize;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::any::Any;

pub use derive::Kind;

type ConstructResult<K> = Result<K, <K as Kind>::Error>;

pub trait Kind: Sized + Send + 'static {
    type ConstructItem: Serialize + DeserializeOwned + Send + Unpin + 'static;
    type Error: Send;
    type ConstructFuture: Future<Output = ConstructResult<Self>> + Send + 'static;

    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture;

    type DeconstructItem: Serialize + DeserializeOwned + Send + Unpin + 'static;
    type DeconstructFuture: Future<Output = ()> + Send + 'static;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture;
}

pub(crate) trait SerdeAny: erased_serde::Serialize + mopa::Any + Send {
    fn as_any(self) -> Box<dyn Any>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

mopafy!(SerdeAny);

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + mopa::Any + Send {}
