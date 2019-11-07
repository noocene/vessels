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

#[cfg(not(any(feature = "core", target_feature = "atomics")))]
#[doc(hidden)]
mod sequential_inner;

pub(crate) fn new_executor() -> Result<Executor, super::UnimplementedError> {
    #[cfg(not(any(feature = "core", target_feature = "atomics")))]
    return Ok(Box::new(sequential_inner::Executor::new()));
    #[cfg(not(not(any(feature = "core", target_feature = "atomics"))))]
    return;
}
