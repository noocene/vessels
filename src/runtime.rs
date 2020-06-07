use crate::{resource::ResourceError, Convert, CoreError, Resource, Sha256};
use core::{convert::Infallible, marker::PhantomData};
use core_error::Error as StdError;
use core_futures_io::{AsyncRead, AsyncWrite};
use futures::Future;

use thiserror::Error;

#[derive(Debug, Error)]
#[bounds(where
    T: StdError + 'static,
    V: StdError + 'static,
    W: StdError + 'static,
    X: StdError + 'static,
    U: StdError + 'static
)]
pub enum RuntimeError<T, U, V, W, X> {
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
    #[error("read error: {0}")]
    Read(#[source] U),
    #[error("write error: {0}")]
    Write(#[source] V),
    #[error("flush error: {0}")]
    Flush(#[source] W),
    #[error("close error: {0}")]
    Close(#[source] X),
}

impl<T, U, V, W, X> From<CoreError> for RuntimeError<T, U, V, W, X> {
    fn from(input: CoreError) -> Self {
        RuntimeError::Core(input)
    }
}

impl<T, U, V, W, X> From<ResourceError<Infallible>> for RuntimeError<T, U, V, W, X> {
    fn from(input: ResourceError<Infallible>) -> Self {
        RuntimeError::Resource(input)
    }
}

pub trait Runtime<T: AsyncWrite, U: AsyncRead> {
    type Instance: Future<
        Output = Result<
            (),
            RuntimeError<Self::Error, U::Error, T::WriteError, T::FlushError, T::CloseError>,
        >,
    >;
    type Error;

    fn instantiate(&mut self, module: WasmResource, writer: T, reader: U) -> Self::Instance;
}

pub type WasmResource = Resource<Wasm, Convert, Sha256>;

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
