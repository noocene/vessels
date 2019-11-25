mod queue;
mod task;
use super::Spawner;
use crate::kind::Future;

pub(crate) struct Executor;

impl Spawner for Executor {
    fn spawn_boxed(&mut self, future: Future<()>) {
        task::Task::spawn(future)
    }
    fn run_boxed(&mut self, future: Future<()>) {
        task::Task::spawn(future)
    }
}

impl Executor {
    pub fn new() -> Box<dyn Spawner> {
        Box::new(Executor)
    }
}
