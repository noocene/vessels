mod array;
mod collections;
mod functions;
mod future;
mod option;
mod phantom_data;
mod primitives;
mod result;
pub mod serde;
mod stream;
mod tuple;
mod unit;
pub mod using;
pub use self::serde::Serde;

use futures::Future as IFuture;

use crate::Kind;

pub trait AsKindMarker {}

pub trait AsKind<M: AsKindMarker>: Sized {
    type Kind: Kind;

    type ConstructFuture: IFuture<Output = Result<Self, <Self::Kind as Kind>::ConstructError>>
        + Send;

    fn into_kind(self) -> Self::Kind;
    fn from_kind(future: <Self::Kind as Kind>::ConstructFuture) -> Self::ConstructFuture;
}
