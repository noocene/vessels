mod queue;
mod task;
use super::Executor;
use futures::future::LocalBoxFuture;

pub(crate) struct Spawner;

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: LocalBoxFuture<'static, ()>) {
        task::Task::spawn(future)
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner
    }
}
