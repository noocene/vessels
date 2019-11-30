use futures::Future;

#[cfg(target_arch = "wasm32")]
mod web_sequential;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod native;

pub fn spawn<F: Sync + Send + 'static + Future<Output = ()>>(future: F) {
    #[cfg(target_arch = "wasm32")]
    web_sequential::spawn(future);
    #[cfg(not(target_arch = "wasm32"))]
    native::spawn(future);
}
pub fn run<F: Sync + Send + 'static + Future<Output = ()>>(future: F) {
    #[cfg(target_arch = "wasm32")]
    web_sequential::spawn(future);
    #[cfg(not(target_arch = "wasm32"))]
    native::run(future);
}
