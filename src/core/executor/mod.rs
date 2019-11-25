use crate::kind::Future;
use futures::Future as IFuture;

pub(crate) trait Spawner: Sync + Send {
    fn spawn_boxed(&mut self, fut: Future<()>);
    fn run_boxed(&mut self, fut: Future<()>);
}

pub struct Executor(Box<dyn Spawner>);

impl Executor {
    pub fn spawn<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F) {
        self.0.spawn_boxed(Box::pin(future))
    }
    pub fn run<F: Sync + Send + 'static + IFuture<Output = ()>>(&mut self, future: F) {
        self.0.run_boxed(Box::pin(future))
    }
}

#[cfg(target_arch = "wasm32")]
mod web_sequential;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native;

pub(crate) fn new_executor() -> Result<Executor, super::UnimplementedError> {
    #[cfg(target_arch = "wasm32")]
    return Ok(Executor(web_sequential::Executor::new()));
    #[cfg(not(target_arch = "wasm32"))]
    return Ok(Executor(native::Executor::new()));
}
