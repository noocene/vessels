use crate::{resource::ResourceError, Convert, CoreError, Resource, Sha256};
use core::{convert::Infallible, marker::PhantomData, pin::Pin};
use core_error::Error;
use core_futures_io::{AsyncRead, AsyncWrite};
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    stream::Map,
    task::{LocalSpawn, LocalSpawnExt, Spawn, SpawnError, SpawnExt},
    Future, StreamExt, TryFuture, TryFutureExt,
};
use thiserror::Error;

#[derive(Debug, Error)]
#[bounds(where
    T: Error + 'static,
    V: Error + 'static,
    W: Error + 'static,
    X: Error + 'static,
    U: Error + 'static
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
pub type ModuleResource<T> = Resource<Module<T>, Convert, Sha256>;

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

impl<T> From<ModuleResource<T>> for WasmResource {
    fn from(resource: ModuleResource<T>) -> Self {
        Resource::new(resource.hash())
    }
}

impl<T> From<WasmResource> for ModuleResource<T> {
    fn from(resource: WasmResource) -> Self {
        ModuleResource::new(resource.hash())
    }
}

pub struct Module<T> {
    pub binary: Wasm,
    ty: PhantomData<T>,
}

impl<T> From<Vec<u8>> for Module<T> {
    fn from(binary: Vec<u8>) -> Self {
        Module {
            binary: Wasm(binary),
            ty: PhantomData,
        }
    }
}

impl<T> From<Module<T>> for Vec<u8> {
    fn from(module: Module<T>) -> Vec<u8> {
        module.binary.0
    }
}

#[derive(Error, Debug)]
#[bounds(where
    T: Error + 'static,
    U: Error + 'static,
)]
pub enum CoalesceFramedError<T, U> {
    #[error("error spawning future: {0}")]
    Spawn(#[source] SpawnError),
    #[error("runtime error: {0}")]
    Runtime(#[source] T),
    #[error("transport error: {0}")]
    Transport(#[source] U),
}

impl<T, U> From<SpawnError> for CoalesceFramedError<T, U> {
    fn from(input: SpawnError) -> Self {
        CoalesceFramedError::Spawn(input)
    }
}

pub trait CoalesceFramed:
    Runtime<
    FrameAdapterSink<Sender<Vec<u8>>>,
    FrameAdapterStream<Map<Receiver<Vec<u8>>, fn(Vec<u8>) -> Result<Vec<u8>, Infallible>>>,
>
{
    fn coalesce_framed<
        T,
        S: Spawn,
        W: erasure_traits::FramedTransportCoalesce<
            T,
            Map<Receiver<Vec<u8>>, fn(Vec<u8>) -> Result<Vec<u8>, Infallible>>,
            Sender<Vec<u8>>,
            S,
        >,
    >(
        &mut self,
        spawner: S,
        resource: ModuleResource<T>,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        T,
                        CoalesceFramedError<
                            RuntimeError<
                                Self::Error,
                                <FrameAdapterStream<
                                    Map<
                                        Receiver<Vec<u8>>,
                                        fn(Vec<u8>) -> Result<Vec<u8>, Infallible>,
                                    >,
                                > as AsyncRead>::Error,
                                <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::WriteError,
                                <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::FlushError,
                                <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::CloseError,
                            >,
                            <W::Coalesce as TryFuture>::Error,
                        >,
                    >,
                > + Send,
        >,
    >
    where
        Self::Instance: Send + 'static,
        Self::Error: Send + 'static,
        W::Coalesce: Unpin + Send,
        S: Send + 'static,
        T: Send,
        <W::Coalesce as TryFuture>::Error: Send,
    {
        let (a_sender, a_receiver) = channel(0);
        let (b_sender, b_receiver) = channel(0);

        let fut = self.instantiate(
            resource.into(),
            FrameAdapterSink::new(a_sender),
            FrameAdapterStream::new(b_receiver.map(Ok)),
        );

        let handle = spawner.spawn(async move {
            // TODO find a way to properly propagate these errors

            let _ = fut.await;
        });

        Box::pin(async move {
            handle?;

            W::coalesce(a_receiver.map(Ok), b_sender, spawner)
                .map_err(CoalesceFramedError::Transport)
                .await
        })
    }

    fn coalesce_framed_local<
        T,
        S: LocalSpawn,
        W: erasure_traits::FramedTransportCoalesce<
            T,
            Map<Receiver<Vec<u8>>, fn(Vec<u8>) -> Result<Vec<u8>, Infallible>>,
            Sender<Vec<u8>>,
            S,
        >,
    >(
        &mut self,
        spawner: S,
        resource: ModuleResource<T>,
    ) -> Pin<
        Box<
            dyn Future<
                Output = Result<
                    T,
                    CoalesceFramedError<
                        RuntimeError<
                            Self::Error,
                            <FrameAdapterStream<
                                Map<Receiver<Vec<u8>>, fn(Vec<u8>) -> Result<Vec<u8>, Infallible>>,
                            > as AsyncRead>::Error,
                            <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::WriteError,
                            <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::FlushError,
                            <FrameAdapterSink<Sender<Vec<u8>>> as AsyncWrite>::CloseError,
                        >,
                        <W::Coalesce as TryFuture>::Error,
                    >,
                >,
            >,
        >,
    >
    where
        Self::Instance: 'static,
        Self::Error: 'static,
        W::Coalesce: Unpin,
        S: 'static,
    {
        let (a_sender, a_receiver) = channel(0);
        let (b_sender, b_receiver) = channel(0);

        let fut = self.instantiate(
            resource.into(),
            FrameAdapterSink::new(a_sender),
            FrameAdapterStream::new(b_receiver.map(Ok)),
        );

        let handle = spawner.spawn_local(async move {
            // TODO find a way to properly propagate these errors

            let _ = fut.await;
        });

        Box::pin(async move {
            handle?;

            W::coalesce(a_receiver.map(Ok), b_sender, spawner)
                .map_err(CoalesceFramedError::Transport)
                .await
        })
    }
}

impl<
        T: Runtime<
            FrameAdapterSink<Sender<Vec<u8>>>,
            FrameAdapterStream<Map<Receiver<Vec<u8>>, fn(Vec<u8>) -> Result<Vec<u8>, Infallible>>>,
        >,
    > CoalesceFramed for T
{
}

mod adapters;
pub use adapters::{
    FrameAdapter, FrameAdapterSink, FrameAdapterStream, RawAdapter, RawAdapterReader,
    RawAdapterWriter,
};
