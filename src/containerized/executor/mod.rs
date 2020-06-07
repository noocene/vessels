mod queue;
mod task;

use core::future::Future;

pub fn spawn<F: Send + 'static + Future<Output = ()>>(future: F) {
    task::Task::spawn(Box::pin(future));
}
