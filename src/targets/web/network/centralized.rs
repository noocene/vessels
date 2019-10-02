use futures::{
    future::err, lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};

use std::sync::Arc;

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};

use crate::network::{
    centralized::{
        socket::{ConnectConfig, ListenConfig},
        Server,
    },
    DataChannel,
};

use failure::Error;

use wasm_bindgen::{prelude::*, JsCast};

pub(crate) fn listen(_: ListenConfig) -> impl Future<Item = Server, Error = Error> {
    err(failure::err_msg("Socket server functionality is unavailable on the web target"))
}

struct Channel {
    socket: web_sys::WebSocket,
    task: Arc<AtomicTask>,
    receiver: Receiver<Vec<u8>>,
}

impl Channel {
    fn create(socket: web_sys::WebSocket) -> Box<dyn DataChannel> {
        let (sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let mut channel = Channel {
            receiver,
            task,
            socket,
        };
        channel.initialize(sender);
        Box::new(channel)
    }
    fn initialize(&mut self, sender: Sender<Vec<u8>>) {
        self.socket
            .set_binary_type(web_sys::BinaryType::Arraybuffer);
        let task = self.task.clone();
        let sender = sender;
        let on_message_closure = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            let buffer = js_sys::Uint8Array::new(&e.data());
            let mut data = vec![0u8; buffer.length() as usize];
            buffer.copy_to(&mut data);
            sender.send(data).unwrap();
            task.clone().notify();
        }) as Box<dyn FnMut(_)>);
        self.socket
            .set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        on_message_closure.forget();
    }
}

impl DataChannel for Channel {}

impl Sink for Channel {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(
        &mut self,
        mut item: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.socket.send_with_u8_array(&mut item).unwrap();
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

impl Stream for Channel {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(data) => Ok(Async::Ready(Some(data))),
            Err(err) => match err {
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
                TryRecvError::Disconnected => panic!("Client connection channel disconnected!"),
            },
        }
    }
}

struct Connection {
    socket: web_sys::WebSocket,
    task: Arc<AtomicTask>,
}

impl Connection {
    fn connect(
        config: ConnectConfig,
    ) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
        lazy(move || {
            let socket = web_sys::WebSocket::new(&format!(
                "ws://{}:{}",
                config.address.ip(),
                config.address.port()
            ))
            .unwrap();
            let mut connection = Connection {
                socket,
                task: Arc::new(AtomicTask::new()),
            };
            connection.initialize();
            connection
        })
    }
    fn initialize(&mut self) {
        let c_task = self.task.clone();
        let o_task = self.task.clone();
        let on_close_closure = Closure::wrap(Box::new(move || c_task.notify()) as Box<dyn Fn()>);
        let on_open_closure = Closure::wrap(Box::new(move || o_task.notify()) as Box<dyn Fn()>);

        self.socket
            .set_onclose(Some(on_close_closure.as_ref().unchecked_ref()));
        self.socket
            .set_onopen(Some(on_open_closure.as_ref().unchecked_ref()));

        on_close_closure.forget();
        on_open_closure.forget();
    }
}

impl Future for Connection {
    type Item = Box<dyn DataChannel + 'static>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.socket.ready_state() {
            0 => {
                self.task.register();
                Ok(Async::NotReady)
            }
            2 => Ok(Async::NotReady),
            1 => Ok(Async::Ready(Channel::create(self.socket.clone()))),
            3 => Err(failure::err_msg("Connection failed")),
            _ => panic!("Invalid socket state"),
        }
    }
}

pub(crate) fn connect(
    config: ConnectConfig,
) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
    Connection::connect(config)
}
