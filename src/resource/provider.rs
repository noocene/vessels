use super::hash::Algorithm;
use futures::{Future, TryFuture, TryFutureExt};
use protocol::protocol;
use std::{marker::PhantomData, pin::Pin};

#[protocol]
pub trait ResourceProvider<A: Algorithm> {
    type Fetch: TryFuture<Ok = Option<Vec<u8>>>;

    fn fetch(&self, hash: <A as Algorithm>::Hash) -> Self::Fetch;
}

struct ResourceProviderEraser<A: Algorithm, T: ResourceProvider<A>> {
    provider: T,
    algo: PhantomData<A>,
}

impl<A: Algorithm, T: ResourceProvider<A>> ResourceProvider<A> for ResourceProviderEraser<A, T>
where
    T::Fetch: Unpin + Send + 'static,
    <T::Fetch as TryFuture>::Error: 'static + core_error::Error + Send,
{
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
    fn erase(self) -> ErrorErasedResourceProvider<A>
    where
        Self: Sized,
        Self::Fetch: Unpin + Send + 'static,
        Self: Send + 'static,
        A: Send + 'static,
        <Self::Fetch as TryFuture>::Error: core_error::Error + Send,
    {
        Box::new(ResourceProviderEraser {
            provider: self,
            algo: PhantomData,
        })
    }
}

impl<A: Algorithm, T: ResourceProvider<A>> ResourceProviderExt<A> for T {}

pub type ErasedResourceProvider<A, E> = Box<
    dyn ResourceProvider<
            A,
            Fetch = Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, E>> + Send>>,
        > + Send,
>;

pub type ErrorErasedResourceProvider<A> =
    ErasedResourceProvider<A, Box<dyn core_error::Error + Send>>;
