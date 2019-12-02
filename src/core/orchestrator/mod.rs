use crate::{
    channel::IdChannel,
    core::{data::Resource, Constructor, Handle},
    format::{ApplyDecode, Cbor},
    kind::{using, Future, SinkStream},
    object,
    replicate::{Share, Shared},
    Kind,
};

use failure::{Error, Fail};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[cfg(feature = "core")]
mod containers;

use containers::{ConcreteContainers, Containers};

#[derive(Serialize, Deserialize, Kind)]
pub struct Module<T: Kind>(Vec<u8>, PhantomData<T>);

impl<T: Kind> Module<T> {
    pub fn new(data: Vec<u8>) -> Self {
        Module(data, PhantomData)
    }
}

#[derive(Serialize, Deserialize, Kind)]
#[kind(using::Serde)]
struct LocalModule(<ConcreteContainers as Containers>::Module);

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
    pub fn new() -> Orchestrator {
        Orchestrator(Shared::new(Box::new(ConcreteContainers::new())))
    }
}

impl OrchestratorInner for ConcreteContainers {
    fn compile(&self, source: Vec<u8>) -> Future<Result<LocalModule, CompileError>> {
        let compile = Containers::compile(self, source);
        Box::pin(async move { Ok(LocalModule(compile.await)) })
    }
    fn instantiate(
        &self,
        module: LocalModule,
    ) -> Future<Result<SinkStream<Vec<u8>, Error, Vec<u8>>, InstantiateError>> {
        let instantiate = Containers::instantiate(self, &module.0);
        Box::pin(async move {
            let (sink, stream) = instantiate.await.split();
            Ok(SinkStream::new(sink.sink_map_err(Error::from), stream))
        })
    }
}
