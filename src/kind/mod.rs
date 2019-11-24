mod array;
mod collections;
mod default;
mod failure_error;
mod functions;
mod future;
mod iterator;
mod option;
mod phantom_data;
mod primitives;
mod result;
mod serde;
mod sink;
mod sink_stream;
mod stream;
mod tuple;
mod unit;
mod url;
pub mod using;
mod wrapped;
pub use self::serde::Serde;
pub use default::Default;
pub use iterator::Iterator;
pub use sink_stream::SinkStream;

use failure::Fail;
use futures::{Future as IFuture, FutureExt, Sink as ISink, Stream as IStream, StreamExt};
use std::pin::Pin;

use crate::{channel::ChannelError, Kind};

pub type Stream<T> = Pin<Box<dyn IStream<Item = T> + Sync + Send>>;
pub type Future<T> = Pin<Box<dyn IFuture<Output = T> + Sync + Send>>;
pub type Sink<T, E> = Pin<Box<dyn ISink<T, Error = E> + Sync + Send>>;

pub trait Flatten: Sized {
    fn flatten<F: IFuture<Output = Self> + Sync + Send + 'static>(fut: F) -> Self;
}

impl<T> Flatten for Future<T> {
    fn flatten<F: IFuture<Output = Self> + Sync + Send + 'static>(fut: F) -> Self {
        Box::pin(fut.flatten())
    }
}

impl<T> Flatten for Stream<T> {
    fn flatten<F: IFuture<Output = Self> + Sync + Send + 'static>(fut: F) -> Self {
        Box::pin(fut.into_stream().flatten())
    }
}

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
