use futures::Future;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
mod web_executor {
    use futures::{
        executor::{Notify, NotifyHandle, Spawn},
        Async, Future,
    };
    use std::sync::{Arc, Mutex};
    use stdweb::web::set_timeout;

    pub(crate) struct Notifier<T> {
        pub(crate) handle: Mutex<Option<NotifyHandle>>,
        pub(crate) task: Arc<Mutex<Spawn<T>>>,
    }
    unsafe impl<T> Send for Notifier<T> {}
    unsafe impl<T> Sync for Notifier<T> {}
    impl<T: 'static> Notify for Notifier<T>
    where
        T: Future<Item = (), Error = ()>,
    {
        fn notify(&self, _: usize) {
            let task = self.task.clone();
            let handle = self.handle.lock().unwrap().as_ref().unwrap().clone();
            set_timeout(
                move || {
                    let val = task.lock().unwrap().poll_future_notify(&handle, 0);
                    match val {
                        Ok(Async::Ready(_)) => (),
                        Ok(Async::NotReady) => (),
                        Err(_) => (),
                    }
                },
                0,
            );
        }
    }
}

/// Runs the provided future using an appropriate platform-specific executor.
pub fn run<F>(future: F)
where
    F: Future<Item = (), Error = ()> + Send + 'static,
{
    #[cfg(any(target_arch = "linux", target_arch = "macos", target_arch = "linux"))]
    tokio::run(future);
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    {
        use futures::executor::{spawn, NotifyHandle};
        use std::sync::{Arc, Mutex};

        use web_executor::Notifier;
        let task = spawn(future);
        let notifier = Arc::new(Notifier {
            handle: Mutex::new(None),
            task: Arc::new(Mutex::new(task)),
        });
        let notify_handle = NotifyHandle::from(notifier.clone());
        *notifier.handle.lock().unwrap() = Some(notify_handle.clone());
        notify_handle.notify(0);
    };
}
