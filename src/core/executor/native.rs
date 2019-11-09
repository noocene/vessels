use super::Executor;
use futures::{executor::ThreadPool, future::BoxFuture};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: ThreadPool = ThreadPool::new().unwrap();
}

pub(crate) struct Spawner {
    inner: ThreadPool,
}

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: BoxFuture<'static, ()>) {
        self.inner.spawn_ok(future)
    }
    fn run_boxed(&mut self, future: BoxFuture<'static, ()>) {
        self.inner.run(future)
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner {
            inner: POOL.clone(),
        }
    }
}
