use futures::{future::BoxFuture, Future};

pub trait Executor {
    fn spawn_boxed(&mut self, fut: BoxFuture<'static, ()>);
    fn run_boxed(&mut self, fut: BoxFuture<'static, ()>);
}

pub trait Spawn {
    fn spawn<F: Send + 'static + Future<Output = ()>>(&mut self, future: F);
    fn run<F: Send + 'static + Future<Output = ()>>(&mut self, future: F);
}

impl Spawn for Box<dyn Executor> {
    fn spawn<F: Send + 'static + Future<Output = ()>>(&mut self, future: F) {
        self.spawn_boxed(Box::pin(future));
    }
    fn run<F: Send + 'static + Future<Output = ()>>(&mut self, future: F) {
        self.run_boxed(Box::pin(future));
    }
}

#[cfg(target_arch = "wasm32")]
mod web_sequential;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native;

pub(crate) fn new_executor() -> Result<Box<dyn Executor>, super::UnimplementedError> {
    #[cfg(target_arch = "wasm32")]
    return Ok(Box::new(web_sequential::Spawner::new()));
    #[cfg(not(target_arch = "wasm32"))]
    return Ok(Box::new(native::Spawner::new()));
}
