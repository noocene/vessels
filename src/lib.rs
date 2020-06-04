use anyhow::Error;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
    convert::Infallible,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    future::{ready, FutureObj, Ready},
    lock::Mutex,
    task::{Spawn, SpawnError},
    TryFutureExt,
};
use serde::de::DeserializeOwned;
use serde_cbor::{from_slice, Error as CborError};
use std::{collections::HashMap, error::Error as StdError, sync::Arc};
use thiserror::Error;

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

pub trait Algorithm {
    type Hash;
}

pub struct Sha256Sum(pub [u8; 32]);

pub struct Sha256;

impl Algorithm for Sha256 {
    type Hash = Sha256Sum;
}

pub trait Rehydrate<T>: Sized {
    type Error;
    type Rehydrate: Future<Output = Result<T, Self::Error>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate;
}

pub struct Resource<T, U: Rehydrate<T>, A: Algorithm>(A::Hash, PhantomData<(T, U)>);

pub struct Cbor;

impl<T: DeserializeOwned> Rehydrate<T> for Cbor {
    type Error = CborError;
    type Rehydrate = Ready<Result<T, Self::Error>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate {
        ready(from_slice(&data))
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

pub struct Convert;

impl<T: From<Vec<u8>>> Rehydrate<T> for Convert {
    type Error = Infallible;
    type Rehydrate = Ready<Result<T, core::convert::Infallible>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate {
        ready(Ok(data.into()))
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
        E: StdError + Send + Sync + 'static,
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
                            .map_err(From::from),
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
    E: StdError + Sync + Send + 'static,
>(
    cb: F,
) -> impl Future<Output = Result<(), CoreError>> {
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
