use futures::future::BoxFuture;

pub trait Spawner {
    fn spawn(&mut self, fut: BoxFuture<'static, ()>);
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
