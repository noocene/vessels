mod array;
mod collections;
mod default;
mod error;
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

use anyhow::Error;
use core::pin::Pin;
use futures::{
    stream::once, Future as IFuture, FutureExt, Sink as ISink, Stream as IStream, StreamExt,
};
use std::error::Error as StdError;
use thiserror::Error;

use crate::{channel::ChannelError, Kind};

#[derive(Error, Kind, Debug)]
#[error("transport error: {cause}")]
pub struct TransportError {
    #[source]
    cause: Error,
}

impl TransportError {
    fn new(cause: Error) -> Self {
        TransportError { cause }
    }
}

pub type Future<T> = Pin<Box<dyn IFuture<Output = T> + Sync + Send>>;
pub type Fallible<T, E> = Future<Result<T, E>>;
pub type Stream<T> = Pin<Box<dyn IStream<Item = T> + Sync + Send>>;
pub type Infallible<T> = Fallible<T, TransportError>;
pub type Sink<T, E> = Pin<Box<dyn ISink<T, Error = E> + Sync + Send>>;

/// The result of reconstructing a Kind.
pub type ConstructResult<K> = Result<K, <K as Kind>::ConstructError>;
/// The result of deconstructing a Kind.
pub type DeconstructResult<K> = Result<(), <K as Kind>::DeconstructError>;

pub trait Flatten: Sized {
    fn flatten<
        E: 'static + Sync + Send + Into<Error>,
        F: IFuture<Output = Result<Self, E>> + Sync + Send + 'static,
    >(
        fut: F,
    ) -> Self;
}

impl<U: From<TransportError> + Sync + Send, T> Flatten for Fallible<T, U> {
    fn flatten<
        E: 'static + Sync + Send + Into<Error>,
        F: IFuture<Output = Result<Self, E>> + Sync + Send + 'static,
    >(
        fut: F,
    ) -> Self {
        Box::pin(async move {
            fut.await
                .map_err(|e| U::from(TransportError::new(e.into())))?
                .await
        })
    }
}

impl<U: From<TransportError>, T> Flatten for Stream<Result<T, U>> {
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
                    Err(e) => Box::pin(once(
                        async move { Err(U::from(TransportError::new(e.into()))) },
                    )) as Stream<Result<T, U>>,
                    Ok(s) => Box::pin(s),
                }
            }
            .into_stream()
            .flatten(),
        )
    }
}

pub trait AsKindMarker {}

#[derive(Error, Debug)]
pub enum WrappedError<T: StdError + 'static> {
    #[error("{0}")]
    Concrete(#[from] T),
    #[error("got {got} items in construct, expected {expected}")]
    Insufficient { got: usize, expected: usize },
    #[error("failed to send on underlying channel: {0}")]
    Send(ChannelError),
}

pub trait AsKind<M: AsKindMarker>: Sized {
    type Kind: Kind;

    fn into_kind(self) -> Self::Kind;
    fn from_kind(kind: Self::Kind) -> Self;
}
