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
use std::fmt::Debug;

pub use derive::{object, Kind};

#[doc(hidden)]
pub use futures;
#[doc(hidden)]
pub use serde;

pub type ConstructResult<K> = Result<K, <K as Kind>::ConstructError>;
pub type DeconstructResult<K> = Result<(), <K as Kind>::DeconstructError>;

pub trait Kind: Sized + Send + 'static {
    type ConstructItem: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static;
    type ConstructError: Debug + Send;
    type ConstructFuture: Future<Output = ConstructResult<Self>> + Send + 'static;

    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture;

    type DeconstructItem: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static;
    type DeconstructError: Debug + Send;
    type DeconstructFuture: Future<Output = DeconstructResult<Self>> + Send + 'static;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture;
}

pub(crate) trait SerdeAny: erased_serde::Serialize + mopa::Any + Send {}

mopafy!(SerdeAny);

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + mopa::Any + Send {}
