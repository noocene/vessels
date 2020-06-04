use crate::resource::manager::ResourceManager;
use anyhow::Error;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
    convert::{Infallible, TryFrom, TryInto},
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    future::{ready, FutureObj, Ready},
    lock::Mutex,
    stream::FuturesUnordered,
    task::{Spawn, SpawnError},
    StreamExt, TryFutureExt,
};
use ring::digest::{digest, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_cbor::{from_slice, to_vec, Error as CborError};
use std::{collections::HashMap, error::Error as StdError, hash::Hash, sync::Arc};
use thiserror::Error;

pub mod resource;
#[doc(inline)]
pub use resource::Resource;

use resource::{
    hash::{Algorithm, Hasher},
    Rehydrate,
};

#[macro_export]
macro_rules! with_core {
    ($core:expr => $block:block) => {{
        let _inner_guard = unsafe { $crate::_inner_use_core(Some($core)) };
        {
            $block
        };
        drop(_inner_guard);
    };};
}

pub struct Ring {
    data: Vec<u8>,
}

impl Hasher<Sha256> for Ring {
    fn new() -> Self {
        Ring { data: vec![] }
    }

    fn write(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data)
    }

    fn hash(&self) -> Sha256Sum {
        let hash = digest(&SHA256, &self.data);
        let mut sum = [0u8; 32];
        sum.copy_from_slice(hash.as_ref());
        Sha256Sum(sum)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Sha256Sum(pub [u8; 32]);

#[derive(Clone, Copy)]
pub struct Sha256;

impl Algorithm for Sha256 {
    type Hash = Sha256Sum;
}

pub struct Cbor;

impl<T: DeserializeOwned + Serialize> Rehydrate<T> for Cbor {
    type RehydrateError = CborError;
    type Rehydrate = Ready<Result<T, Self::RehydrateError>>;
    type DumpError = CborError;
    type Dump = Ready<Result<Vec<u8>, Self::DumpError>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate {
        ready(from_slice(&data))
    }
    fn dump(data: T) -> Self::Dump {
        ready(to_vec(&data))
    }
}

pub struct Module<T> {
    pub binary: Vec<u8>,
    ty: PhantomData<T>,
}

impl<T> From<Vec<u8>> for Module<T> {
    fn from(binary: Vec<u8>) -> Self {
        Module {
            binary,
            ty: PhantomData,
        }
    }
}

impl<T> From<Module<T>> for Vec<u8> {
    fn from(module: Module<T>) -> Vec<u8> {
        module.binary
    }
}

pub struct Convert;

impl<T: TryFrom<Vec<u8>> + TryInto<Vec<u8>>> Rehydrate<T> for Convert {
    type RehydrateError = <T as TryFrom<Vec<u8>>>::Error;
    type Rehydrate = Ready<Result<T, Self::RehydrateError>>;
    type DumpError = <T as TryInto<Vec<u8>>>::Error;
    type Dump = Ready<Result<Vec<u8>, Self::DumpError>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate {
        ready(data.try_into())
    }
    fn dump(data: T) -> Self::Dump {
        ready(data.try_into())
    }
}

type ModuleResource<T> = Resource<Module<T>, Convert, Sha256>;

pub trait FutureTypeConstructor<T> {
    type Future: Future<Output = T>;
}

pub trait Runtime {
    type Instantiate;
    type Error;

    fn instantiate<T>(
        &mut self,
        module: ModuleResource<T>,
    ) -> <Self::Instantiate as FutureTypeConstructor<Result<T, Self::Error>>>::Future
    where
        Self::Instantiate: FutureTypeConstructor<Result<T, Self::Error>>;
}

pub struct Core {
    singleton: Singleton,
}

impl Core {
    pub fn new() -> Self {
        Core {
            singleton: Singleton {
                local: Arc::new(Mutex::new(HashMap::new())),
            },
        }
    }
}

pub trait Provider {
    fn acquire(
        &self,
        ty: TypeId,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Box<dyn Any + Send>>, Error>> + Send>>;

    fn box_clone(&self) -> Box<dyn Provider + Send>;
}

pub struct Singleton {
    local: Arc<
        Mutex<
            HashMap<
                TypeId,
                Box<
                    dyn Fn() -> Pin<
                            Box<dyn Future<Output = Result<Box<dyn Any + Send>, Error>> + Send>,
                        > + Send,
                >,
            >,
        >,
    >,
}

impl Singleton {
    fn private_clone(&self) -> Self {
        Singleton {
            local: self.local.clone(),
        }
    }

    fn acquire<T: Any>(&self) -> impl Future<Output = Result<Option<T>, Error>> {
        let this = self.local.clone();

        async move {
            let this = this.lock().await;

            if let Some(call) = this.get(&TypeId::of::<T>()) {
                Ok(Some(
                    *Box::<dyn Any + Send>::downcast::<T>((call)().await?).unwrap(),
                ))
            } else {
                Ok(None)
            }
        }
    }

    fn register<
        T: Any + Send,
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        E: Into<Error> + 'static,
    >(
        &self,
        cb: F,
    ) -> impl Future<Output = Result<(), Error>> {
        let this = self.local.clone();

        async move {
            let mut this = this.lock().await;

            this.insert(
                TypeId::of::<T>(),
                Box::new(move || {
                    Box::pin(
                        (cb)()
                            .map_ok(|item| Box::new(item) as Box<dyn Any + Send>)
                            .map_err(Into::into),
                    )
                }),
            );

            Ok(())
        }
    }
}

thread_local! {
    static CURRENT_SINGLETON: RefCell<Option<Singleton>> = RefCell::new(None);
}

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("error from core: {0}")]
    Error(
        #[source]
        #[from]
        Error,
    ),
    #[error("no active core")]
    NoCore,
}

fn get_singleton() -> Option<Singleton> {
    let mut ptr: Option<Singleton> = None;

    CURRENT_SINGLETON.with(|singleton| {
        singleton.borrow().as_ref().map(|singleton| {
            ptr = Some(singleton.private_clone());
        });
    });

    ptr
}

pub fn acquire<T: Any>() -> impl Future<Output = Result<Option<T>, CoreError>> {
    let singleton = get_singleton().ok_or(CoreError::NoCore);

    async { Ok(singleton?.acquire::<T>().await?) }
}

pub fn register<
    T: Any + Send,
    F: Fn() -> Fut + Send + 'static,
    Fut: Future<Output = Result<T, E>> + Send + 'static,
    E: 'static,
>(
    cb: F,
) -> impl Future<Output = Result<(), CoreError>>
where
    Error: From<E>,
{
    let singleton = get_singleton().ok_or(CoreError::NoCore);

    async move { Ok(singleton?.register(cb).await?) }
}

fn use_singleton(new_singleton: Option<Singleton>) {
    CURRENT_SINGLETON.with(|singleton| {
        *singleton.borrow_mut() = new_singleton;
    });
}

pub struct CoreGuard(Option<Singleton>);

impl Drop for CoreGuard {
    fn drop(&mut self) {
        use_singleton(self.0.take())
    }
}

#[doc(hidden)]
pub unsafe fn _inner_use_core(core: Option<&Core>) -> CoreGuard {
    let guard = get_singleton()
        .map(|singleton| CoreGuard(Some(singleton)))
        .unwrap_or(CoreGuard(None));

    use_singleton(core.map(|core| core.singleton.private_clone()));

    guard
}

pub struct CorePreserver<T: Spawn>(pub T);

struct CoreTask<F: Future> {
    future: F,
    core: Option<Core>,
}

impl<F: Future> CoreTask<F> {
    fn new(future: F, core: Option<Core>) -> Self {
        CoreTask { future, core }
    }
}

impl<F: Future + Unpin> Future for CoreTask<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let guard = unsafe { _inner_use_core(self.core.as_ref()) };
        let output = Pin::new(&mut self.future).poll(cx);
        drop(guard);
        output
    }
}

impl<T: Spawn> Spawn for CorePreserver<T> {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        let core = get_singleton().map(|singleton| Core { singleton });
        self.0
            .spawn_obj(Box::pin(CoreTask::new(future, core)).into())
    }
}

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

#[derive(Clone)]
pub struct SimpleResourceManager {
    providers: Arc<
        Mutex<
            HashMap<
                TypeId,
                Vec<
                    Box<
                        dyn Fn(
                                Box<dyn Any + Send>,
                            ) -> Pin<
                                Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>,
                            > + Sync
                            + Send,
                    >,
                >,
            >,
        >,
    >,
}

impl ResourceManager for SimpleResourceManager {
    type Fetch =
        Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, ResourceError<Infallible>>> + Send>>;

    fn fetch(
        &self,
        algo: TypeId,
        mut hash: Box<dyn FnMut() -> Box<dyn Any + Send> + Send>,
    ) -> Self::Fetch {
        let providers = self.providers.clone();

        Box::pin(async move {
            let providers = providers.lock().await;

            let providers = providers
                .get(&algo)
                .ok_or(ResourceError::UnknownAlgorithm)?
                .as_slice();

            let mut fetch = providers
                .iter()
                .map(|provider| (provider)(hash()))
                .collect::<FuturesUnordered<_>>();

            while let Some(item) = fetch.next().await {
                if let Some(item) = item? {
                    return Ok(Some(item));
                }
            }

            Ok(None)
        })
    }
}

impl SimpleResourceManager {
    pub fn new() -> Self {
        SimpleResourceManager {
            providers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_provider<A: Algorithm + Any, T: ResourceProvider<A>>(
        &mut self,
        provider: T,
    ) -> impl Future<Output = ()>
    where
        T: Sync + Sized,
        T::Fetch: Unpin + Send + 'static,
        T: Send + 'static,
        A: Send + 'static,
        Error: From<T::Error>,
    {
        let providers = self.providers.clone();

        async move {
            let mut providers = providers.lock().await;

            providers
                .entry(TypeId::of::<A>())
                .or_insert(vec![])
                .push(Box::new(move |any| {
                    let fut = provider.fetch(*Box::<dyn Any>::downcast(any).unwrap());

                    Box::pin(async move { fut.await.map_err(From::from) })
                }));
        }
    }
}

pub struct MemoryStore<A: Algorithm> {
    data: Arc<Mutex<HashMap<A::Hash, Vec<u8>>>>,
}

impl<A: Algorithm> Clone for MemoryStore<A> {
    fn clone(&self) -> Self {
        MemoryStore {
            data: self.data.clone(),
        }
    }
}

impl<A: Algorithm> MemoryStore<A> {
    pub fn new() -> Self {
        MemoryStore {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn intern<H: Hasher<A>, T, U: Rehydrate<T>>(
        &mut self,
        item: T,
    ) -> impl Future<Output = Result<Resource<T, U, A>, Error>>
    where
        A::Hash: Eq + Hash + Clone,
        Error: From<U::DumpError>,
    {
        let data = self.data.clone();

        async move {
            let mut data = data.lock().await;

            let item = U::dump(item).await?;

            let mut hasher = H::new();

            hasher.write(&item);

            let hash = hasher.hash();

            data.insert(hash.clone(), item);

            Ok(Resource::new(hash))
        }
    }
}

impl<A: Algorithm> ResourceProvider<A> for MemoryStore<A>
where
    A::Hash: Hash + Eq + Send + 'static,
{
    type Error = Error;
    type Fetch = Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>, Error>> + Send>>;

    fn fetch(&self, hash: A::Hash) -> Self::Fetch {
        let data = self.data.clone();

        Box::pin(async move {
            let data = data.lock().await;

            Ok(data.get(&hash).cloned())
        })
    }
}
