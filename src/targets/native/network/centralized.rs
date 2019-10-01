use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread::spawn;

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};

use futures::{lazy, task::AtomicTask, Async, AsyncSink, Future, Poll, Sink, StartSend, Stream};

use ws::{CloseCode, Factory, Handler, Handshake, Message, WebSocket};

use failure::Error;

use crate::network::{
    centralized::socket::{self, ConnectConfig, ListenConfig},
    DataChannel,
};

struct ConnectionHandler {
    c_sender: Sender<Vec<u8>>,
    send: Option<Box<dyn FnOnce() + Send>>,
    task: Arc<AtomicTask>,
}

pub(crate) struct Connection {
    sender: ws::Sender,
    receiver: Receiver<Vec<u8>>,
    task: Arc<AtomicTask>,
    send_task: Arc<AtomicTask>,
    queue_size: Arc<AtomicUsize>,
}

impl Connection {
    fn create(
        sender: ws::Sender,
        r_sender: Sender<Connection>,
        r_task: Arc<AtomicTask>,
    ) -> ConnectionHandler {
        let (c_sender, receiver) = unbounded();
        let task = Arc::new(AtomicTask::new());
        let send: Box<dyn FnOnce() + Send> = {
            let sender = sender.clone();
            let task = task.clone();
            Box::new(move || {
                r_sender
                    .send(Connection {
                        receiver,
                        send_task: Arc::new(AtomicTask::new()),
                        queue_size: Arc::new(AtomicUsize::new(0)),
                        sender,
                        task: task.clone(),
                    })
                    .expect("Server connection channel disconnected!");
                r_task.notify();
            })
        };
        ConnectionHandler {
            c_sender,
            send: Some(send),
            task,
        }
    }
    fn connect(
        config: ConnectConfig,
    ) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
        lazy(move || {
            let (sender, receiver) = unbounded();
            let task = Arc::new(AtomicTask::new());
            let mut socket = WebSocket::new(ConnectionFactory::new(sender, task.clone())).unwrap();
            socket
                .connect(
                    format!("ws://{}:{}", config.address.ip(), config.address.port())
                        .parse()
                        .unwrap(),
                )
                .expect("TODO implement failure on connection");
            spawn(move || socket.run());
            ClientConnection::wait(receiver, task)
        })
    }
}

struct ClientConnection {
    task: Arc<AtomicTask>,
    receiver: Receiver<Connection>,
}

impl ClientConnection {
    fn wait(
        receiver: Receiver<Connection>,
        task: Arc<AtomicTask>,
    ) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
        ClientConnection { task, receiver }
    }
}

impl Future for ClientConnection {
    type Item = Box<dyn DataChannel + 'static>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.receiver.try_recv() {
            Ok(connection) => Ok(Async::Ready(Box::new(connection))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Server connection channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl Handler for ConnectionHandler {
    fn on_message(&mut self, message: Message) -> ws::Result<()> {
        self.c_sender
            .send(match message {
                Message::Binary(data) => data,
                Message::Text(_) => message.into_data(),
            })
            .unwrap();
        self.task.notify();
        Ok(())
    }
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        let mut send = None;
        std::mem::swap(&mut send, &mut self.send);
        let send = send.unwrap();
        send();
        Ok(())
    }
    fn on_close(&mut self, _code: CloseCode, _reason: &str) {}
}

impl Stream for Connection {
    type Item = Vec<u8>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(data) => Ok(Async::Ready(Some(data))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Server connection channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl Sink for Connection {
    type SinkItem = Vec<u8>;
    type SinkError = Error;
    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.queue_size.fetch_add(1, Ordering::SeqCst);
        let send_task = self.send_task.clone();
        let sender = self.sender.clone();
        let queue_size = self.queue_size.clone();
        tokio::spawn(lazy(move || {
            sender.send(item).unwrap();
            if queue_size.fetch_sub(1, Ordering::SeqCst) == 0 {
                send_task.notify();
            };
            Ok(())
        }));
        Ok(AsyncSink::Ready)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        if self.queue_size.load(Ordering::SeqCst) == 0 {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

impl DataChannel for Connection {}

pub(crate) struct Server {
    receiver: Receiver<Connection>,
    task: Arc<AtomicTask>,
}

impl Server {
    fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
        lazy(move || {
            let (sender, receiver) = unbounded();
            let task = Arc::new(AtomicTask::new());
            let socket = WebSocket::new(ConnectionFactory::new(sender, task.clone())).unwrap();
            let server = Server { receiver, task };
            let socket = socket
                .bind(config.address)
                .map_err(|_| Error::address_in_use())?;
            spawn(move || socket.run());
            let server: socket::Server = Box::new(server);
            Ok(server)
        })
    }
}

struct ConnectionFactory {
    sender: Sender<Connection>,
    task: Arc<AtomicTask>,
}

impl ConnectionFactory {
    fn new(sender: Sender<Connection>, task: Arc<AtomicTask>) -> ConnectionFactory {
        ConnectionFactory { sender, task }
    }
}

impl Factory for ConnectionFactory {
    type Handler = ConnectionHandler;
    fn connection_made(&mut self, ws: ws::Sender) -> Self::Handler {
        Connection::create(ws, self.sender.clone(), self.task.clone())
    }
}

impl Stream for Server {
    type Item = Box<dyn DataChannel>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(connection) => Ok(Async::Ready(Some(Box::new(connection)))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Server connection channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

pub(crate) fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
    Server::listen(config)
}

pub(crate) fn connect(
    config: ConnectConfig,
) -> impl Future<Item = Box<dyn DataChannel + 'static>, Error = Error> {
    Connection::connect(config)
}
