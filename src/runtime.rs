use crate::{
    acquire,
    resource::{ErasedResourceManager, ResourceError, ResourceManagerExt},
    Convert, CoreError, Resource, Sha256,
};
use core::{
    convert::Infallible,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use core_error::Error as StdError;
use futures::{task::AtomicWaker, Future, Sink, Stream, TryStream};
use std::sync::Arc;
use thiserror::Error;
use wasmer_runtime::{
    error::Error as WasmError, func, imports, instantiate, Func, Global, Instance, Memory,
};

#[derive(Debug, Error)]
#[bounds(where T: StdError + 'static)]
pub enum InstantiateError<T> {
    #[error("failed to acquire binary")]
    NoBinary,
    #[error("no active resource manager")]
    NoResourceManager,
    #[error("core error: {0}")]
    Core(#[source] CoreError),
    #[error("runtime error: {0}")]
    Runtime(#[source] T),
    #[error("resource error: {0}")]
    Resource(#[source] ResourceError<Infallible>),
}

impl<T> From<CoreError> for InstantiateError<T> {
    fn from(input: CoreError) -> Self {
        InstantiateError::Core(input)
    }
}

impl<T> From<ResourceError<Infallible>> for InstantiateError<T> {
    fn from(input: ResourceError<Infallible>) -> Self {
        InstantiateError::Resource(input)
    }
}

pub trait Runtime {
    type Instance: TryStream<Ok = Vec<u8>> + Sink<Vec<u8>>;
    type Instantiate: Future<
        Output = Result<Self::Instance, InstantiateError<Self::InstantiateError>>,
    >;
    type InstantiateError;

    fn instantiate(&mut self, module: WasmResource) -> Self::Instantiate;
}

type WasmResource = Resource<Wasm, Convert, Sha256>;
// type ModuleResource<T> = Resource<Module<T>, Convert, Sha256>;

pub struct Wasm(pub Vec<u8>);

impl From<Vec<u8>> for Wasm {
    fn from(binary: Vec<u8>) -> Self {
        Wasm(binary)
    }
}

impl From<Wasm> for Vec<u8> {
    fn from(module: Wasm) -> Vec<u8> {
        module.0
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

pub struct WasmerRuntime;

pub struct WasmerInstance {
    instance: Instance,
    buf_addr: usize,
    memory: Memory,
}

impl Sink<Vec<u8>> for WasmerInstance {
    type Error = WasmError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
    fn start_send(self: Pin<&mut Self>, item: Vec<u8>) -> Result<(), Self::Error> {
        todo!()
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}

impl Stream for WasmerInstance {
    type Item = Result<Vec<u8>, WasmError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        todo!()
    }
}

impl Runtime for WasmerRuntime {
    type Instance = WasmerInstance;
    type Instantiate = Pin<
        Box<
            dyn Future<Output = Result<Self::Instance, InstantiateError<Self::InstantiateError>>>
                + Send,
        >,
    >;
    type InstantiateError = WasmError;

    fn instantiate(&mut self, module: WasmResource) -> Self::Instantiate {
        Box::pin(async move {
            let manager: ErasedResourceManager = acquire()
                .await?
                .ok_or(InstantiateError::NoResourceManager)?;

            let fetch = manager.fetch(module);
            let data = fetch.await?.ok_or(InstantiateError::NoBinary)?.0;

            let instance = instantiate(
                &data,
                &imports! {
                    "env" => {
                        "_vessel_wake" => func!(move || {
                            let a: Instance = panic!();

                            a.exports.get::<Func<(), ()>>("_vessel_poll").unwrap().call().unwrap();
                        })
                    }
                },
            )
            .map_err(InstantiateError::Runtime)?;

            println!("{:?}", instance.exports().collect::<Vec<_>>());

            let memory = instance
                .exports
                .get::<Memory>("memory")
                .map_err(|e| InstantiateError::Runtime(WasmError::ResolveError(e)))?;

            let buf_addr = instance
                .exports
                .get::<Global>("_VESSEL_BUFFER")
                .map_err(|e| InstantiateError::Runtime(WasmError::ResolveError(e)))?
                .get()
                .to_u128() as usize;

            Ok(WasmerInstance {
                instance,
                buf_addr,
                memory,
            })
        })
    }
}
