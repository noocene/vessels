mod queue;
mod task;
use super::Executor;
use crate::kind::Future;

pub(crate) struct Spawner;

impl Executor for Spawner {
    fn spawn_boxed(&mut self, future: Future<()>) {
        task::Task::spawn(future)
    }
    fn run_boxed(&mut self, future: Future<()>) {
        task::Task::spawn(future)
    }
}

impl Spawner {
    pub fn new() -> Self {
        Spawner
    }
}
