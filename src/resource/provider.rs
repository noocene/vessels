use super::hash::Algorithm;
use anyhow::Error;
use futures::{Future, TryFutureExt};
use std::{marker::PhantomData, pin::Pin};

pub trait ResourceProvider<A: Algorithm> {
    type Error;
    type Fetch: Future<Output = Result<Option<Vec<u8>>, Self::Error>>;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch;
}

struct ResourceProviderEraser<A: Algorithm, T: ResourceProvider<A>> {
    provider: T,
    algo: PhantomData<A>,
}

impl<A: Algorithm, T: ResourceProvider<A>> ResourceProvider<A> for ResourceProviderEraser<A, T>
where
    T::Fetch: Unpin + Send + 'static,
    T::Error: 'static,
    Error: From<T::Error>,
{
    type Error = Error;
    type Fetch = Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>>;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch {
        Box::pin(self.provider.fetch(hash).map_err(From::from))
    }
}

pub trait ResourceProviderExt<A: Algorithm>: ResourceProvider<A> {
    fn erase(self) -> ErasedResourceProvider<A>
    where
        Self: Sized,
        Self::Fetch: Unpin + Send + 'static,
        Self: Send + 'static,
        A: Send + 'static,
        Error: From<Self::Error>,
    {
        Box::new(ResourceProviderEraser {
            provider: self,
            algo: PhantomData,
        })
    }
}

impl<A: Algorithm, T: ResourceProvider<A>> ResourceProviderExt<A> for T {}

pub type ErasedResourceProvider<A> = Box<
    dyn ResourceProvider<
            A,
            Error = Error,
            Fetch = Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>>,
        > + Send,
>;
