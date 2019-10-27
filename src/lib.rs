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

pub use derive::{kind, Kind};

pub trait Kind: Sized + Send + 'static {
    type ConstructItem: Serialize + DeserializeOwned + Send + 'static;
    type ConstructFuture: Future<Item = Self> + Send + 'static;

    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture;

    type DeconstructItem: Serialize + DeserializeOwned + Send + 'static;
    type DeconstructFuture: Future<Item = ()> + Send + 'static;

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
