use futures::{
    future::err, lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream,
};

use stdweb::{
    unstable::TryInto,
    web::{
        event::{SocketCloseEvent, SocketMessageEvent, SocketOpenEvent},
        ArrayBuffer, SocketBinaryType, SocketReadyState, WebSocket,
    },
};

use std::sync::{Arc, RwLock};

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};

use crate::errors::Error;
use crate::network::{
    centralized::{
        socket::{ConnectConfig, ListenConfig},
        Server,
    },
    DataChannel,
};

pub(crate) fn listen(_: ListenConfig) -> impl Future<Item = Server, Error = Error> {
    err(Error::feature_unavailable())
}

struct Channel {
    socket: WebSocket,
    task: Arc<RwLock<AtomicTask>>,
    receiver: Receiver<Vec<u8>>,
}

impl Channel {
    fn create(socket: WebSocket) -> Box<dyn DataChannel> {
        let (sender, receiver) = unbounded();
        let task = Arc::new(RwLock::new(AtomicTask::new()));
        let mut channel = Channel {
            receiver,
            task,
            socket,
        };
        channel.initialize(sender);
        Box::new(channel)
    }
    fn initialize(&mut self, sender: Sender<Vec<u8>>) {
        self.socket.set_binary_type(SocketBinaryType::ArrayBuffer);
        let task = self.task.clone();
        let sender = sender;
        let on_message = move |e: SocketMessageEvent| {
            let buffer: ArrayBuffer = js! { return @{&e}.data; }
                .try_into()
                .expect("Buffer conversion failed");
            sender.send(Vec::<u8>::from(buffer)).unwrap();
            task.clone().read().unwrap().notify();
        };
        js! {
            @{&self.socket}.onmessage = @{on_message};
        }
    }
}

impl DataChannel for Channel {}

impl Sink for Channel {
    type SinkItem = Vec<u8>;
    type SinkError = Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.socket.send_bytes(item.as_slice()).unwrap();
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
                    self.task.read().unwrap().register();
                    Ok(Async::NotReady)
                }
                TryRecvError::Disconnected => panic!("Client connection channel disconnected!"),
            },
        }
    }
}

struct Connection {
    socket: WebSocket,
    task: Arc<RwLock<AtomicTask>>,
}

impl Connection {
    fn connect(
        config: ConnectConfig,
    ) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
        lazy(move || {
            let socket = WebSocket::new(&format!(
                "ws://{}:{}",
                config.address.ip(),
                config.address.port()
            ))
            .unwrap();
            let mut connection = Connection {
                socket,
                task: Arc::new(RwLock::new(AtomicTask::new())),
            };
            connection.initialize();
            connection
        })
    }
    fn initialize(&mut self) {
        let c_task = self.task.clone();
        let o_task = self.task.clone();
        let on_close = move |_: SocketCloseEvent| {
            c_task.read().unwrap().notify();
        };
        let on_open = move |_: SocketOpenEvent| {
            o_task.read().unwrap().notify();
        };
        js! {
            @{&self.socket}.onclose = @{on_close};
            @{&self.socket}.onopen = @{on_open};
        }
    }
}

impl Future for Connection {
    type Item = Box<dyn DataChannel + 'static>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.socket.ready_state() {
            SocketReadyState::Connecting => {
                self.task.read().unwrap().register();
                Ok(Async::NotReady)
            }
            SocketReadyState::Closing => Ok(Async::NotReady),
            SocketReadyState::Open => Ok(Async::Ready(Channel::create(self.socket.clone()))),
            SocketReadyState::Closed => Err(Error::connection_failed()),
        }
    }
}

pub(crate) fn connect(
    config: ConnectConfig,
) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
    Connection::connect(config)
}
