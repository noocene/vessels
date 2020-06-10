use anyhow::Error;
use core::{
    any::{Any, TypeId},
    cell::RefCell,
    convert::{TryFrom, TryInto},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    future::{ready, FutureObj, Ready},
    lock::Mutex,
    task::{Spawn, SpawnError},
    TryFutureExt,
};
#[cfg(feature = "ring-sha256")]
use ring::digest::{digest, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_cbor::{from_slice, to_vec, Error as CborError};
use std::{collections::HashMap, hash::Hash, sync::Arc};
use thiserror::Error;

#[doc(hidden)]
pub use futures::task::SpawnExt;

#[cfg(feature = "containerized")]
mod containerized;
#[cfg(feature = "containerized")]
pub use containerized::{VesselEntry, VesselReader, VesselWriter};
#[cfg(feature = "containerized")]
#[doc(hidden)]
pub use containerized::{_vessel_entry_construct, _vessel_unravel};

pub mod runtime;

pub mod resource;
#[doc(inline)]
pub use resource::Resource;

mod memory_store;
pub use memory_store::MemoryStore;

mod simple_resource_manager;
pub use simple_resource_manager::SimpleResourceManager;

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

#[cfg(feature = "ring-sha256")]
pub struct Ring {
    data: Vec<u8>,
}

#[cfg(feature = "ring-sha256")]
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
                        > + Sync
                        + Send,
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
        F: Fn() -> Fut + Sync + Send + 'static,
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
    F: Fn() -> Fut + Sync + Send + 'static,
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
