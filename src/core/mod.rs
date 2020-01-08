use alloc::sync::Arc;
use anyhow::Error;
#[cfg(target_arch = "wasm32")]
use core::any::Any;
use futures::{lock, SinkExt, StreamExt};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
use thiserror::Error;

use crate::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::{Fallible, FromTransportError, Future, Infallible, SinkStream},
    object,
    replicate::Share,
    Kind, OnTo,
};

mod executor;
pub use executor::{run, spawn};

pub mod data;
pub mod hal;
pub mod orchestrator;

#[doc(hidden)]
pub type Constructor<T> = Box<dyn FnOnce(Handle) -> Infallible<T> + Send + Sync>;

#[derive(Error, Debug, Kind)]
#[error("{feature} is unimplemented on this target")]
pub struct UnimplementedError {
    feature: String,
}

#[derive(Error, Debug, Kind)]
pub enum CoreError {
    #[error("feature unavailable or unregistered")]
    Unavailable,
    #[error("`{0}`")]
    Unimplemented(#[source] UnimplementedError),
    #[error("`handle transfer failed: {0}`")]
    Construct(#[source] Error),
    #[error("`underlying transport failed: {0}`")]
    Transport(#[source] Error),
}

impl FromTransportError for CoreError {
    fn from_transport_error(error: Error) -> Self {
        CoreError::Transport(error)
    }
}

#[doc(hidden)]
pub struct Logger(());

impl Logger {
    pub fn info(&self, _message: String) {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        web_sys::console::log_1(&_message.into());
        #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
        unimplemented!();
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", _message);
    }
}

lazy_static! {
    #[doc(hidden)]
    pub static ref LOG: Logger = Logger(());
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
lazy_static! {
    static ref HANDLE: Mutex<(
        Option<Handle>,
        HashMap<[u8; 32], Box<dyn Fn() -> Box<dyn Any + Sync + Send> + Sync + Send>>
    )> = Mutex::new((None, HashMap::new()));
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
#[doc(hidden)]
pub fn register_handle(item: Handle) {
    let mut handle = HANDLE.lock().unwrap();
    if handle.0.is_none() {
        handle.0 = Some(item)
    }
}

pub fn acquire<K: Kind>() -> Fallible<K, CoreError> {
    #[cfg(feature = "core")]
    {
        if let Some(item) = LOCAL_CORE
            .lock()
            .unwrap()
            .get(&K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD)
        {
            let item = Ok(*Box::<dyn Any>::downcast((item)()).unwrap());
            return Box::pin(async move { item });
        }
    }
    #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
    return {
        let handle = HANDLE.lock().unwrap();
        if let Some(item) = handle.1.get(&K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD) {
            let item = Ok(*Box::<dyn Any>::downcast((item)()).unwrap());
            return Box::pin(async move { item });
        }
        if let Some(handle) = &handle.0 {
            handle.acquire::<K>()
        } else {
            Box::pin(async { Err(CoreError::Unavailable) })
        }
    };
    #[cfg(not(all(target_arch = "wasm32", not(feature = "core"))))]
    Box::pin(async { Err(CoreError::Unavailable) })
}

#[object]
trait HandleInner {
    fn acquire(&self, ty: [u8; 32]) -> Fallible<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>;
}

#[cfg(feature = "core")]
lazy_static! {
    pub static ref LOCAL_CORE: Mutex<HashMap<[u8; 32], Box<dyn Fn() -> Box<dyn Any + Sync + Send> + Sync + Send>>> =
        Mutex::new(HashMap::new());
}

#[derive(Kind)]
pub struct Handle(Box<dyn HandleInner>);

impl Handle {
    pub fn acquire<K: Kind>(&self) -> Future<Result<K, CoreError>> {
        let channel = self.0.acquire(K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD);
        Box::pin(async move {
            channel
                .await?
                .sink_map_err(CoreError::Construct)
                .decode::<IdChannel, Cbor>()
                .await
                .map_err(|e: K::ConstructError| CoreError::Construct(e.into()))
        })
    }
}

pub struct Core {
    capabilities: Arc<
        Mutex<
            HashMap<
                [u8; 32],
                Box<
                    dyn Fn() -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>>
                        + Sync
                        + Send,
                >,
            >,
        >,
    >,
}

impl HandleInner for Core {
    fn acquire(
        &self,
        ty: [u8; 32],
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>> {
        if let Some(capability) = self.capabilities.lock().unwrap().get(&ty) {
            capability()
        } else {
            Box::pin(async move { Err(CoreError::Unavailable) })
        }
    }
}

pub fn register<K: Kind>(item: impl Fn() -> K + Sync + Send + 'static) {
    #[cfg(feature = "core")]
    {
        LOCAL_CORE.lock().unwrap().insert(
            K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD,
            Box::new(move || Box::new(item())),
        );
    }
    #[cfg(all(not(feature = "core"), target_arch = "wasm32"))]
    {
        HANDLE.lock().unwrap().1.insert(
            K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD,
            Box::new(move || Box::new(item())),
        );
    }
}

impl Core {
    pub fn new() -> Self {
        Core {
            capabilities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn register<K: Kind>(&mut self, item: impl Fn() -> K + Sync + Send + 'static) {
        let item = Arc::new(lock::Mutex::new(item));
        self.capabilities.lock().unwrap().insert(
            K::USE_KIND_MACRO_TO_GENERATE_THIS_FIELD,
            Box::new(move || {
                let item = item.clone();
                Box::pin(async move {
                    let (sink, stream) = (item.lock().await)()
                        .on_to::<IdChannel>()
                        .await
                        .encode::<Cbor>()
                        .split();
                    Ok(SinkStream::new(sink.sink_map_err(Error::from), stream))
                })
            }),
        );
    }
    pub fn into_handle(self) -> Handle {
        Handle(Box::new(self))
    }
}

impl Share for Core {
    fn share(&self) -> Self {
        Core {
            capabilities: self.capabilities.clone(),
        }
    }
}
