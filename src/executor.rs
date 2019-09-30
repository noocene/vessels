#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
use crate::targets;

use futures::Future;

/// Runs the provided future using an appropriate platform-specific executor.
pub fn run<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    tokio::run(future);
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::executor::spawn(future);
}

/// Spawns the provided future on the currently running executor.
pub fn spawn<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    tokio::spawn(future);
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::executor::spawn(future);
}
