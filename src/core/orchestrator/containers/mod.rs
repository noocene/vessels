use futures::{future::LocalBoxFuture, Sink, Stream};

#[cfg(target_arch = "wasm32")]
pub mod web;

pub trait Instance: Stream<Item = Vec<u8>> + Sink<Vec<u8>> {}

pub trait Containers {
    type Module;
    type Instance: Instance;

    fn compile<T: AsRef<[u8]>>(&mut self, data: T) -> LocalBoxFuture<'static, Self::Module>;
    fn instantiate(&mut self, module: &Self::Module) -> LocalBoxFuture<'static, Self::Instance>;
}
