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
    handle: Mutex<Option<NotifyHandle>>,
    task: Arc<Mutex<Spawn<T>>>,
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
        handle: Mutex::new(None),
        task: Arc::new(Mutex::new(task)),
    });
    let notify_handle = NotifyHandle::from(notifier.clone());
    *notifier.handle.lock().unwrap() = Some(notify_handle.clone());
    notify_handle.notify(0);
}
