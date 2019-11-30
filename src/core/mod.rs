use failure::{Error, Fail};
use futures::{Future as IFuture, SinkExt, StreamExt};
use lazy_static::lazy_static;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{self, Display, Formatter},
    mem::transmute,
    sync::{Arc, Mutex},
};

use crate::{
    channel::IdChannel,
    format::{ApplyDecode, ApplyEncode, Cbor},
    kind::{Future, SinkStream},
    object,
    replicate::Share,
    Kind, OnTo,
};

mod executor;
pub use executor::{run, spawn};

pub mod hal;
pub mod orchestrator;

pub type Constructor<T> = Box<dyn FnOnce(Handle) -> Future<T> + Send + Sync>;

#[derive(Fail, Debug, Kind)]
#[fail(display = "{} is unimplemented on this target", feature)]
pub struct UnimplementedError {
    feature: String,
}

#[derive(Fail, Debug, Kind)]
pub enum CoreError {
    Unavailable,
    Unimplemented(#[fail(cause)] UnimplementedError),
    Construct(#[fail(cause)] Error),
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use CoreError::{Construct, Unavailable, Unimplemented};
        write!(
            formatter,
            "{}",
            match self {
                Unavailable => "this feature is unavailable or unregistered".to_owned(),
                Unimplemented(feature) => format!("{}", feature),
                Construct(e) => format!("handle transfer failed: {}", e),
            }
        )
    }
}

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
    pub static ref LOG: Logger = Logger(());
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
lazy_static! {
    static ref HANDLE: Mutex<Option<Handle>> = Mutex::new(None);
}

#[cfg(all(target_arch = "wasm32", not(feature = "core")))]
pub fn register_handle(item: Handle) {
    let mut handle = HANDLE.lock().unwrap();
    if handle.is_none() {
        *handle = Some(item)
    }
}

pub fn acquire<K: Any + Kind>() -> Future<Result<K, CoreError>> {
    #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
    return {
        let handle = HANDLE.lock().unwrap();
        if let Some(handle) = &*handle {
            handle.acquire::<K>()
        } else {
            Box::pin(async { Err(CoreError::Unavailable) })
        }
    };
    #[cfg(not(all(target_arch = "wasm32", not(feature = "core"))))]
    Box::pin(async { Err(CoreError::Unavailable) })
}

#[derive(Kind)]
pub struct Identifier(u64);

impl Identifier {
    pub fn new<T: Any>() -> Self {
        let item = format!("{:?}", TypeId::of::<T>());
        Identifier(item.split_whitespace().nth(3).unwrap().parse().unwrap())
    }
}

impl From<Identifier> for TypeId {
    fn from(input: Identifier) -> Self {
        unsafe { transmute(input) }
    }
}

#[object]
trait HandleInner {
    fn acquire(
        &self,
        ty: Identifier,
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>>;
}

#[derive(Kind)]
pub struct Handle(Box<dyn HandleInner>);

impl Handle {
    pub fn acquire<K: Any + Kind>(&self) -> Future<Result<K, CoreError>> {
        let channel = self.0.acquire(Identifier::new::<K>());
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
                TypeId,
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
        ty: Identifier,
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, CoreError>> {
        if let Some(capability) = self.capabilities.lock().unwrap().get(&ty.into()) {
            capability()
        } else {
            Box::pin(async move { Err(CoreError::Unavailable) })
        }
    }
}

impl Core {
    pub fn new() -> Self {
        Core {
            capabilities: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn register<K: Kind + Any + Share>(&mut self, item: K) {
        self.capabilities.lock().unwrap().insert(
            TypeId::of::<K>(),
            Box::new(move || {
                let item = item.share();
                Box::pin(async move {
                    let (sink, stream) = item.on_to::<IdChannel>().await.encode::<Cbor>().split();
                    Ok(SinkStream::new(sink.sink_map_err(Error::from), stream))
                })
            }),
        );
    }
    pub fn as_handle(self) -> Handle {
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
