mod queue;
mod task;
use super::Executor;
use futures::future::BoxFuture;

pub(crate) struct Spawner;

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: BoxFuture<'static, ()>) {
        task::Task::spawn(future)
    }
    fn run_boxed(&mut self, future: BoxFuture<'static, ()>) {
        task::Task::spawn(future)
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner
    }
}
