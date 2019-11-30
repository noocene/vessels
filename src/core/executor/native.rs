use futures::{
    executor::{block_on, ThreadPool},
    Future,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POOL: ThreadPool = ThreadPool::new().unwrap();
}

pub(crate) fn spawn<F: Sync + Send + 'static + Future<Output = ()>>(future: F) {
    POOL.clone().spawn_ok(future);
}

pub(crate) fn run<F: Sync + Send + 'static + Future<Output = ()>>(future: F) {
    block_on(future)
}
