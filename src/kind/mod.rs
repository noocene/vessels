mod array;
mod collections;
pub mod default;
mod failure_error;
mod functions;
mod future;
pub mod iterator;
mod option;
mod phantom_data;
mod primitives;
mod result;
pub mod serde;
mod sink;
mod stream;
mod tuple;
mod unit;
mod url;
pub mod using;
mod wrapped;
pub use self::serde::Serde;
pub use default::Default;
pub use iterator::Iterator;

use failure::Fail;
use futures::{Future as IFuture, Sink as ISink, Stream as IStream};
use std::pin::Pin;

use crate::{channel::ChannelError, Kind};

pub type Stream<T> = Pin<Box<dyn IStream<Item = T> + Sync + Send>>;
pub type Future<T> = Pin<Box<dyn IFuture<Output = T> + Sync + Send>>;
pub type Sink<T, E> = Pin<Box<dyn ISink<T, Error = E> + Sync + Send>>;

pub trait AsKindMarker {}

#[derive(Fail, Debug)]
pub enum WrappedError<T: Fail> {
    #[fail(display = "{}", _0)]
    Concrete(#[fail(cause)] T),
    #[fail(display = "got {} items in construct, expected {}", got, expected)]
    Insufficient { got: usize, expected: usize },
    #[fail(display = "failed to send on underlying channel: {}", _0)]
    Send(ChannelError),
}

impl<T: Fail> From<T> for WrappedError<T> {
    fn from(input: T) -> Self {
        WrappedError::Concrete(input)
    }
}

impl<T: Fail> From<ChannelError> for WrappedError<T> {
    fn from(input: ChannelError) -> Self {
        WrappedError::Send(input)
    }
}

pub trait AsKind<M: AsKindMarker>: Sized {
    type Kind: Kind;

    fn into_kind(self) -> Self::Kind;
    fn from_kind(kind: Self::Kind) -> Self;
}
