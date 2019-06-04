use std::net::{Ipv4Addr, SocketAddrV4};

use futures::{
    executor::{spawn, Notify, NotifyHandle, Spawn},
    Async, Future, Sink, Stream,
};

use vitruvia::network::centralized::socket::{self, ConnectConfig};

#[macro_use]
extern crate stdweb;

use stdweb::web::set_timeout;

use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

static PORT: u16 = 8080;

struct Notifier<T> {
    me: Mutex<Option<NotifyHandle>>,
    task: Arc<Mutex<Spawn<T>>>,
}

unsafe impl<T> Send for Notifier<T> {}
unsafe impl<T> Sync for Notifier<T> {}
impl<T: 'static> Notify for Notifier<T>
where
    T: Future,
    T::Item: Debug,
    T::Error: Debug,
{
    fn notify(&self, _: usize) {
        // This method is first called at initialization, then later whenever something calls
        // the `Task::notify()` method of the `futures` crate.
        let task = self.task.clone();
        let me = self.me.lock().unwrap().as_ref().unwrap().clone();
        // We use `set_timeout` with a timeout of 0 in order to schedule the closure to be executed
        // immediately after we return.
        set_timeout(
            move || {
                // Calling `poll_future_notify` will poll the future to see whether it's ready.
                // If the future is not ready, we are guaranteed that the task (which is `me` here, and is
                // also the same as `future_task` that we created at the start) is saved somewhere, and
                // `notify` will be called on it later.
                let val = task.lock().unwrap().poll_future_notify(&me, 0);
                match val {
                    Ok(Async::Ready(item)) => println!("finished: {:?}", item), // You decide what to do here
                    Ok(Async::NotReady) => (),
                    Err(err) => console!(log, format!("error: {:?}", err)), // You decide what to do here
                }
            },
            0,
        );
    }
}

fn main() {
    let config: ConnectConfig = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT).into();
    let client = socket::connect(config)
        .map_err(|e| eprintln!("connection failed: {:?}", e))
        .and_then(|connection| {
            println!("connected");
            let (send, receive) = connection.split();
            let send = send
                .send(b"test".to_vec())
                .map_err(|e| console!(log, format!("send failed: {:?}", e)))
                .and_then(|_| Ok(()));
            let receive = receive
                .for_each(|message| {
                    console!(log, format!("{:?}", message));
                    Ok(())
                })
                .map_err(|e| console!(log, format!("recv failed: {:?}", e)))
                .and_then(|_| Ok(()));
            send.join(receive).and_then(|(_, _)| Ok(()))
        });

    let task = spawn(client);

    let notifier = Arc::new(Notifier {
        me: Mutex::new(None),
        task: Arc::new(Mutex::new(task)),
    });
    let notify_handle = NotifyHandle::from(notifier.clone());
    *notifier.me.lock().unwrap() = Some(notify_handle.clone());
    notify_handle.notify(0);
}
