use super::{hash::Algorithm, Rehydrate};
use crate::{
    resource::{
        provider::{ErasedResourceProvider, ResourceProvider},
        ResourceError,
    },
    Resource,
};
use futures::{
    future::{ready, AndThen, Either, MapErr, MapOk, Ready},
    Future, TryFuture, TryFutureExt,
};
use std::{
    any::{Any, TypeId},
    convert::Infallible,
    pin::Pin,
};

pub trait ResourceManager {
    type Fetch: Future<Output = Result<Option<Vec<u8>>, ResourceError<Infallible>>>;

    fn fetch(
        &self,
        algo: TypeId,
        hash: Box<dyn FnMut() -> Box<dyn Any + Send> + Send>,
    ) -> Self::Fetch;
}

impl<T: ?Sized + ResourceManager> ResourceManager for Box<T> {
    type Fetch = T::Fetch;

    fn fetch(
        &self,
        algo: TypeId,
        hash: Box<dyn FnMut() -> Box<dyn Any + Send> + Send>,
    ) -> Self::Fetch {
        T::fetch(self, algo, hash)
    }
}

pub type ErasedResourceManager = Box<
    dyn ResourceManager<
            Fetch = Pin<
                Box<dyn Future<Output = Result<Option<Vec<u8>>, ResourceError<Infallible>>> + Send>,
            >,
        > + Send,
>;

pub struct ResourceManagerEraser<T: ResourceManager> {
    manager: T,
}

impl<T: ResourceManager> ResourceManager for ResourceManagerEraser<T>
where
    T::Fetch: Send + 'static,
{
    type Fetch =
        Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, ResourceError<Infallible>>> + Send>>;

    fn fetch(
        &self,
        algo: TypeId,
        hash: Box<dyn FnMut() -> Box<dyn Any + Send> + Send>,
    ) -> Self::Fetch {
        Box::pin(self.manager.fetch(algo, hash))
    }
}

pub trait ResourceManagerExt: ResourceManager {
    fn into_erased(self) -> ErasedResourceManager
    where
        Self: Sized + Send + 'static,
        Self::Fetch: Send,
    {
        Box::new(ResourceManagerEraser { manager: self })
    }

    fn fetch<A: Algorithm + Any, T, U: Rehydrate<T>>(
        &self,
        resource: Resource<T, U, A>,
    ) -> AndThen<
        MapErr<
            <Self as ResourceManager>::Fetch,
            fn(ResourceError<std::convert::Infallible>) -> ResourceError<U::RehydrateError>,
        >,
        Either<
            MapErr<
                MapOk<U::Rehydrate, fn(T) -> Option<T>>,
                fn(U::RehydrateError) -> ResourceError<U::RehydrateError>,
            >,
            Ready<Result<Option<T>, ResourceError<U::RehydrateError>>>,
        >,
        fn(
            Option<Vec<u8>>,
        ) -> Either<
            MapErr<
                MapOk<U::Rehydrate, fn(T) -> Option<T>>,
                fn(U::RehydrateError) -> ResourceError<U::RehydrateError>,
            >,
            Ready<Result<Option<T>, ResourceError<U::RehydrateError>>>,
        >,
    >
    where
        A::Hash: Clone + Send,
        T: Send + 'static,
        U: Send + 'static,
    {
        ResourceManager::fetch(
            self,
            TypeId::of::<A>(),
            Box::new(move || Box::new(resource.hash())),
        )
        .map_err(
            ResourceError::cast
                as fn(ResourceError<std::convert::Infallible>) -> ResourceError<U::RehydrateError>,
        )
        .and_then(
            (|data| {
                if let Some(data) = data {
                    Either::Left(
                        U::rehydrate(data)
                            .map_ok(Some as fn(T) -> Option<T>)
                            .map_err(
                                ResourceError::Rehydration
                                    as fn(U::RehydrateError) -> ResourceError<U::RehydrateError>,
                            ),
                    )
                } else {
                    Either::Right(ready(Ok(None)))
                }
            })
                as fn(
                    Option<Vec<u8>>,
                ) -> Either<
                    MapErr<_, _>,
                    Ready<Result<Option<T>, ResourceError<U::RehydrateError>>>,
                >,
        )
    }
}

impl<T: ResourceManager> ResourceManagerExt for T {}

pub trait ResourceRegistrant<A, T>
where
    A: Algorithm,
    T: ResourceProvider<A>,
{
    type Register: TryFuture<Ok = ()>;

    fn register_provider(&mut self, provider: T) -> Self::Register;
}

impl<A: Algorithm, R: ResourceProvider<A>, T: ?Sized + ResourceRegistrant<A, R>>
    ResourceRegistrant<A, R> for Box<T>
{
    type Register = T::Register;

    fn register_provider(&mut self, provider: R) -> Self::Register {
        T::register_provider(self, provider)
    }
}

pub type ErasedResourceRegistrant<A, E> = Box<
    dyn ResourceRegistrant<
            A,
            ErasedResourceProvider<A>,
            Register = Pin<Box<dyn Future<Output = Result<(), E>> + Send>>,
        > + Send,
>;

pub type ErrorErasedResourceRegistrant<A> =
    ErasedResourceRegistrant<A, Box<dyn core_error::Error + Send>>;
