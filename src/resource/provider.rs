use super::hash::Algorithm;
use futures::{Future, TryFutureExt};
use std::{marker::PhantomData, pin::Pin};

pub trait ResourceProvider<A: Algorithm> {
    type Error;
    type Fetch: Future<Output = Result<Option<Vec<u8>>, Self::Error>>;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch;
}

impl<A: Algorithm, T: ?Sized + ResourceProvider<A>> ResourceProvider<A> for Box<T> {
    type Error = T::Error;
    type Fetch = T::Fetch;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch {
        T::fetch(self, hash)
    }
}

struct ResourceProviderEraser<A: Algorithm, T: ResourceProvider<A>> {
    provider: T,
    algo: PhantomData<A>,
}

impl<A: Algorithm, T: ResourceProvider<A>> ResourceProvider<A> for ResourceProviderEraser<A, T>
where
    T::Fetch: Unpin + Send + 'static,
    T::Error: 'static + core_error::Error + Send,
{
    type Error = Box<dyn core_error::Error + Send>;
    type Fetch = Pin<
        Box<dyn Future<Output = Result<Option<Vec<u8>>, Box<dyn core_error::Error + Send>>> + Send>,
    >;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch {
        Box::pin(
            self.provider
                .fetch(hash)
                .map_err(|e| Box::new(e) as Box<dyn core_error::Error + Send>),
        )
    }
}

pub trait ResourceProviderExt<A: Algorithm>: ResourceProvider<A> {
    fn erase(self) -> ErasedResourceProvider<A>
    where
        Self: Sized,
        Self::Fetch: Unpin + Send + 'static,
        Self: Send + 'static,
        A: Send + 'static,
        Self::Error: core_error::Error + Send,
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
            Error = Box<dyn core_error::Error + Send>,
            Fetch = Pin<
                Box<
                    dyn Future<Output = Result<Option<Vec<u8>>, Box<dyn core_error::Error + Send>>>
                        + Send,
                >,
            >,
        > + Send,
>;
