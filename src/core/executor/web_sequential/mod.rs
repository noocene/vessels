mod queue;
mod task;

use futures::Future;

pub(crate) fn spawn<F: Sync + Send + 'static + Future<Output = ()>>(future: F) {
    task::Task::spawn(Box::pin(future));
}
