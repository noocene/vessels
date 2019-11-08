use super::Executor;
use futures::{executor::ThreadPool, future::BoxFuture};

pub(crate) struct Spawner {
    inner: ThreadPool,
}

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: BoxFuture<'static, ()>) {
        self.inner.spawn_ok(future)
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner {
            inner: ThreadPool::new().unwrap(),
        }
    }
}
