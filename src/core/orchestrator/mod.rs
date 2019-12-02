use crate::{
    channel::IdChannel,
    core::{
        data::{Checksum, Resource},
        Constructor, Handle, UnimplementedError,
    },
    format::{ApplyDecode, Cbor},
    kind::{using, Future, SinkStream},
    object,
    replicate::{Share, Shared},
    Kind,
};

use failure::{Error, Fail};
use futures::SinkExt;
#[cfg(feature = "core")]
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

#[cfg(all(target_arch = "wasm32", feature = "core"))]
type ConcreteContainers = web::WebContainers;
#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
type ConcreteContainers = native::NativeContainers;

#[derive(Serialize, Deserialize, Kind)]
pub struct Module<T: Kind>(Vec<u8>, PhantomData<T>);

impl<T: Kind> Module<T> {
    pub fn new(data: Vec<u8>) -> Self {
        Module(data, PhantomData)
    }
}

#[derive(Serialize, Deserialize, Kind, Clone)]
#[kind(using::Serde)]
pub(crate) struct LocalModule(pub(crate) Checksum);

#[derive(Fail, Debug, Kind)]
#[fail(display = "compile failed: {}", cause)]
pub struct CompileError {
    cause: Error,
}

#[object]
trait OrchestratorInner {
    fn compile(&self, source: Vec<u8>) -> Future<Result<LocalModule, CompileError>>;
    fn instantiate(
        &self,
        module: LocalModule,
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, InstantiateError>>;
}

#[derive(Kind)]
pub struct Orchestrator(Shared<dyn OrchestratorInner>);

#[derive(Fail, Debug, Kind)]
#[fail(display = "instantiate failed: {}", cause)]
pub struct InstantiateError {
    cause: Error,
}

impl Orchestrator {
    pub fn instantiate<K: Kind>(
        &self,
        module: Resource<Module<K>>,
        handle: Handle,
    ) -> Future<Result<K, InstantiateError>> {
        let inner = self.0.share();
        Box::pin(async move {
            let constructor: Constructor<K> = inner
                .instantiate(
                    inner
                        .compile(module.reify().await.unwrap().0)
                        .await
                        .unwrap(),
                )
                .await
                .unwrap()
                .sink_map_err(|cause| InstantiateError { cause })
                .decode::<IdChannel, Cbor>()
                .await
                .unwrap();
            Ok(constructor(handle).await)
        })
    }
    pub fn new() -> Result<Orchestrator, UnimplementedError> {
        #[cfg(feature = "core")]
        return Ok(Orchestrator(Shared::new(Box::new(
            ConcreteContainers::new(),
        ))));
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "orchestrator".to_owned(),
        });
    }
}

#[cfg(feature = "core")]
impl OrchestratorInner for ConcreteContainers {
    fn compile(&self, source: Vec<u8>) -> Future<Result<LocalModule, CompileError>> {
        let compile = self.compile(source);
        Box::pin(async move { Ok(compile.await) })
    }
    fn instantiate(
        &self,
        module: LocalModule,
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, InstantiateError>> {
        let instantiate = self.instantiate(&module);
        Box::pin(async move {
            let (sink, stream) = instantiate.await.split();
            Ok(SinkStream::new(sink.sink_map_err(Error::from), stream))
        })
    }
}
