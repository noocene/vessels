use futures::{Future, Sink, Stream};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

pub trait Instance: Stream<Item = Vec<u8>> + Sink<Vec<u8>> {}

pub trait Containers {
    type Module: Serialize + DeserializeOwned;
    type Compile: Future<Output = Self::Module>;
    type Instance: Instance;
    type Instantiate: Future<Output = Self::Instance>;

    fn compile<T: AsRef<[u8]>>(&self, data: T) -> Self::Compile;
    fn instantiate(&self, module: &Self::Module) -> Self::Instantiate;
}

#[cfg(target_arch = "wasm32")]
pub type ConcreteContainers = web::WebContainers;
#[cfg(not(target_arch = "wasm32"))]
pub type ConcreteContainers = native::NativeContainers;
