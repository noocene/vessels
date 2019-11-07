mod queue;
mod task;
use super::Spawner;
use futures::future::BoxFuture;

pub(crate) struct Executor {}

impl Spawner for Executor {
    fn spawn(&mut self, future: BoxFuture<'static, ()>) {
        task::Task::spawn(future)
    }
}

impl Executor {
    pub fn new() -> Self {
        Executor {}
    }
}
