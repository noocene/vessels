use super::Spawner;
use futures::{executor::ThreadPool, future::BoxFuture};

pub(crate) struct Executor {
    inner: ThreadPool,
}

impl Spawner for Executor {
    fn spawn_boxed(&mut self, future: BoxFuture<'static, ()>) {
        self.inner.spawn_ok(future)
    }
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            inner: ThreadPool::new().unwrap(),
        }
    }
}
