use anyhow::Error;
use core::marker::PhantomData;
use std::{convert::Infallible, error::Error as StdError};
use thiserror::Error;

mod rehydrate;
pub use rehydrate::Rehydrate;
pub mod hash;
use hash::Algorithm;
pub mod manager;
pub mod provider;
pub use manager::{ErasedResourceManager, ResourceManagerExt};

pub struct Resource<T, U: Rehydrate<T>, A: Algorithm>(A::Hash, PhantomData<(T, U)>);

impl<T, U: Rehydrate<T>, A: Algorithm> Clone for Resource<T, U, A>
where
    A::Hash: Clone,
{
    fn clone(&self) -> Self {
        Resource(self.0.clone(), PhantomData)
    }
}

impl<T, U: Rehydrate<T>, A: Algorithm> Resource<T, U, A> {
    pub fn new(data: A::Hash) -> Self {
        Resource(data, PhantomData)
    }

    pub fn hash(&self) -> A::Hash
    where
        A::Hash: Clone,
    {
        self.0.clone()
    }
}

#[derive(Debug, Error)]
#[bounds(where T: StdError + 'static)]
pub enum ResourceError<T> {
    #[error("error from provider: {0}")]
    Provider(#[source] Error),
    #[error("unknown algorithm")]
    UnknownAlgorithm,
    #[error("rehydration error: {0}")]
    Rehydration(#[source] T),
}

impl ResourceError<Infallible> {
    fn cast<E>(self) -> ResourceError<E> {
        match self {
            ResourceError::Provider(e) => ResourceError::Provider(e),
            ResourceError::Rehydration(_) => panic!(),
            ResourceError::UnknownAlgorithm => ResourceError::UnknownAlgorithm,
        }
    }
}

impl<T> From<Error> for ResourceError<T> {
    fn from(input: Error) -> Self {
        ResourceError::Provider(input)
    }
}
