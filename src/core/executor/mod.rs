use crate::kind::Future;
use futures::Future as IFuture;

pub trait Executor: Sync + Send {
    fn spawn_boxed(&mut self, fut: Future<()>);
    fn run_boxed(&mut self, fut: Future<()>);
}

mod private {
    use super::Executor;

    pub trait Sealed {}

    impl Sealed for Box<dyn Executor> {}
}

pub trait Spawn: private::Sealed {
    fn spawn<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F);
    fn run<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F);
}

impl Spawn for Box<dyn Executor> {
    fn spawn<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F) {
        self.spawn_boxed(Box::pin(future));
    }
    fn run<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F) {
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
