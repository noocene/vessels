use futures::{future::BoxFuture, Future};

pub trait Spawner {
    fn spawn_boxed(&mut self, fut: BoxFuture<'static, ()>);
}

pub trait Spawn {
    fn spawn<F: Send + 'static + Future<Output = ()>>(&mut self, future: F);
}

impl Spawn for Executor {
    fn spawn<F: Send + 'static + Future<Output = ()>>(&mut self, future: F) {
        self.spawn_boxed(Box::pin(future));
    }
}

pub type Executor = Box<dyn Spawner>;

#[cfg(all(not(feature = "core"), target_arch = "wasm32"))]
#[doc(hidden)]
mod sequential_inner;

#[cfg(not(target_arch = "wasm32"))]
#[doc(hidden)]
mod threadpool_inner;

pub(crate) fn new_executor() -> Result<Executor, super::UnimplementedError> {
    #[cfg(all(not(feature = "core"), target_arch = "wasm32"))]
    return Ok(Box::new(sequential_inner::Executor::new()));
    #[cfg(not(target_arch = "wasm32"))]
    return Ok(Box::new(threadpool_inner::Executor::new()));
    #[cfg(all(feature = "core", target_arch = "wasm32"))]
    unimplemented!()
}
