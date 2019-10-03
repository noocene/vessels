#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
use futures::Future;

/// Provides a concurrent threaded executor.
pub mod concurrent {
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
        unimplemented!("Web concurrent executor is unimplemented");
    }

    /// Spawns the provided future on the currently running executor.
    pub fn spawn<F>(future: F)
    where
        F: Future<Item = (), Error = ()> + Send + 'static,
    {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        tokio::spawn(future);
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        unimplemented!("Web concurrent executor is unimplemented");
    }

}

/// Provides a local single-threaded executor.
pub mod consecutive {
    use crate::targets;
    use futures::Future;

    /// Runs the provided future using an appropriate platform-specific executor.
    pub fn run<F>(future: F)
    where
        F: Future<Item = (), Error = ()> + 'static,
    {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        unimplemented!("Native consecutive executor is unimplemented");
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::executor::spawn(future);
    }

    /// Spawns the provided future on the currently running executor.
    pub fn spawn<F>(future: F)
    where
        F: Future<Item = (), Error = ()> + 'static,
    {
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        unimplemented!("Native consecutive executor is unimplemented");
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::executor::spawn(future);
    }

}

/// Spawns the provided future on the best available running executor.
pub fn spawn<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    concurrent::spawn(future);
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    consecutive::spawn(future);
}

/// Runs the provided future on the best available executor.
pub fn run<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    concurrent::run(future);
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    consecutive::run(future);
}
