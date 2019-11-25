use super::Spawner;
use crate::kind::Future;
use futures::executor::{block_on, ThreadPool};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: ThreadPool = ThreadPool::new().unwrap();
}

pub(crate) struct Executor {
    inner: ThreadPool,
}

impl Spawner for Executor {
    fn spawn_boxed(&mut self, future: Future<()>) {
        self.inner.spawn_ok(future)
    }
    fn run_boxed(&mut self, future: Future<()>) {
        block_on(future);
    }
}

impl Executor {
    pub fn new() -> Box<dyn Spawner> {
        Box::new(Executor {
            inner: POOL.clone(),
        })
    }
}
