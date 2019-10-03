use futures::Future;
use tokio::runtime::current_thread::{Runtime, TaskExecutor};

pub(crate) fn run<F>(future: F)
where
    F: Future<Item = (), Error = ()> + 'static,
{
    Runtime::new().unwrap().spawn(future).run().unwrap();
}

pub(crate) fn spawn<F>(future: F)
where
    F: Future<Item = (), Error = ()> + 'static,
{
    TaskExecutor::current()
        .spawn_local(Box::new(future))
        .unwrap();
}
