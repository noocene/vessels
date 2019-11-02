mod array;
mod collections;
pub mod default;
mod functions;
mod future;
pub mod iterator;
mod ops_assign;
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
pub use default::Default;
pub use iterator::Iterator;

use futures::{future::BoxFuture, stream::BoxStream};

use crate::Kind;

pub type Stream<T> = BoxStream<'static, T>;
pub type Future<T> = BoxFuture<'static, T>;

pub trait AsKindMarker {}

pub trait AsKind<M: AsKindMarker>: Sized {
    type Kind: Kind;

    fn into_kind(self) -> Self::Kind;
    fn from_kind(kind: Self::Kind) -> Self;
}
