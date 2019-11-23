use super::Executor;
use crate::kind::Future;
use futures::executor::{block_on, ThreadPool};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: ThreadPool = ThreadPool::new().unwrap();
}

pub(crate) struct Spawner {
    inner: ThreadPool,
}

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: Future<()>) {
        self.inner.spawn_ok(future)
    }
    fn run_boxed(&mut self, future: Future<()>) {
        block_on(future);
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner {
            inner: POOL.clone(),
        }
    }
}
