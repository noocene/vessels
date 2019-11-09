use futures::{Future, Sink, Stream};

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

pub trait Instance: Stream<Item = Vec<u8>> + Sink<Vec<u8>> {}

pub trait Containers {
    type Module;
    type Compile: Future<Output = Self::Module>;
    type Instance: Instance;
    type Instantiate: Future<Output = Self::Instance>;

    fn compile<T: AsRef<[u8]>>(&mut self, data: T) -> Self::Compile;
    fn instantiate(&mut self, module: &Self::Module) -> Self::Instantiate;
}
