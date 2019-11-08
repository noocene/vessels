use futures::{future::LocalBoxFuture, Future};

pub trait Executor {
    fn spawn_boxed(&mut self, fut: LocalBoxFuture<'static, ()>);
}

pub trait Spawn {
    fn spawn<F: 'static + Future<Output = ()>>(&mut self, future: F);
}

impl Spawn for Box<dyn Executor> {
    fn spawn<F: 'static + Future<Output = ()>>(&mut self, future: F) {
        self.spawn_boxed(Box::pin(future));
    }
}

#[cfg(target_arch = "wasm32")]
mod web_sequential;

pub(crate) fn new_executor() -> Result<Box<dyn Executor>, super::UnimplementedError> {
    #[cfg(target_arch = "wasm32")]
    return Ok(Box::new(web_sequential::Spawner::new()));
    #[cfg(not(target_arch = "wasm32"))]
    return Err(super::UnimplementedError {
        feature: "sequential future execution".to_owned(),
    });
}
