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

use core::pin::Pin;
use failure::{Error, Fail};
use futures::{
    stream::once, Future as IFuture, FutureExt, Sink as ISink, Stream as IStream, StreamExt,
};

use crate::{channel::ChannelError, Kind};

pub type Future<T> = Pin<Box<dyn IFuture<Output = T> + Sync + Send>>;
pub type Fallible<T, E> = Future<Result<T, E>>;
pub type Stream<T> = Pin<Box<dyn IStream<Item = T> + Sync + Send>>;
pub type Infallible<T> = Fallible<T, Error>;
pub type Sink<T, E> = Pin<Box<dyn ISink<T, Error = E> + Sync + Send>>;

/// The result of reconstructing a Kind.
pub type ConstructResult<K> = Result<K, <K as Kind>::ConstructError>;
/// The result of deconstructing a Kind.
pub type DeconstructResult<K> = Result<(), <K as Kind>::DeconstructError>;

pub trait FromTransportError: Send + Sync {
    fn from_transport_error(error: Error) -> Self;
}

impl FromTransportError for Error {
    fn from_transport_error(error: Error) -> Self {
        error
    }
}

pub trait Flatten: Sized {
    fn flatten<
        E: 'static + Sync + Send + Into<Error>,
        F: IFuture<Output = Result<Self, E>> + Sync + Send + 'static,
    >(
        fut: F,
    ) -> Self;
}

impl<U: FromTransportError, T> Flatten for Fallible<T, U> {
    fn flatten<
        E: 'static + Sync + Send + Into<Error>,
        F: IFuture<Output = Result<Self, E>> + Sync + Send + 'static,
    >(
        fut: F,
    ) -> Self {
        Box::pin(async move {
            fut.await
                .map_err(|e| U::from_transport_error(e.into()))?
                .await
        })
    }
}

impl<U: FromTransportError, T> Flatten for Stream<Result<T, U>> {
    fn flatten<
        E: 'static + Sync + Send + Into<Error>,
        F: IFuture<Output = Result<Self, E>> + Sync + Send + 'static,
    >(
        fut: F,
    ) -> Self {
        Box::pin(
            async move {
                let r = fut.await;
                match r {
                    Err(e) => Box::pin(once(async move { Err(U::from_transport_error(e.into())) }))
                        as Stream<Result<T, U>>,
                    Ok(s) => Box::pin(s),
                }
            }
                .into_stream()
                .flatten(),
        )
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

pub trait AsKind<M: AsKindMarker>: Sized {
    type Kind: Kind;

    fn into_kind(self) -> Self::Kind;
    fn from_kind(kind: Self::Kind) -> Self;
}
