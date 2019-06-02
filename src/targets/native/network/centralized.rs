use std::sync::{Arc, RwLock};
use std::thread::spawn;

use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};

use futures::{lazy, task::AtomicTask, Async, Future, Poll, Stream};

use ws::{CloseCode, Factory, Handler, Handshake, Message, WebSocket};

use crate::errors::Error;

use crate::network::{
    self,
    centralized::socket::{self, ListenConfig},
    ConnectionStatus,
};

struct ConnectionHandler {
    sender: ws::Sender,
    c_sender: Sender<Vec<u8>>,
    task: Arc<RwLock<AtomicTask>>,
}

pub(crate) struct Connection {
    status: ConnectionStatus,
    details: socket::ConnectionDetails,
    receiver: Receiver<Vec<u8>>,
    task: Arc<RwLock<AtomicTask>>,
}

impl Connection {
    fn new(sender: ws::Sender) -> (Connection, ConnectionHandler) {
        let (c_sender, receiver) = unbounded();
        let task = Arc::new(RwLock::new(AtomicTask::new()));
        (
            Connection {
                receiver,
                status: ConnectionStatus::default(),
                details: socket::ConnectionDetails {
                    address: ListenConfig::from(0u16).address,
                },
                task: task.clone(),
            },
            ConnectionHandler {
                sender,
                c_sender,
                task,
            },
        )
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
        self.task.read().unwrap().notify();
        Ok(())
    }
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        Ok(())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {}
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
                    self.task.read().unwrap().register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl network::Connection for Connection {
    type TransportDetails = socket::ConnectionDetails;
    fn transport_details(&self) -> &Self::TransportDetails {
        &self.details
    }
    fn status(&self) -> ConnectionStatus {
        self.status
    }
}

pub(crate) struct Server {
    receiver: Receiver<Connection>,
    broadcaster: ws::Sender,
    task: Arc<RwLock<AtomicTask>>,
}

impl Server {
    fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
        lazy(move || {
            let (sender, receiver) = unbounded();
            let task = Arc::new(RwLock::new(AtomicTask::new()));
            let socket = WebSocket::new(ConnectionFactory::new(sender, task.clone())).unwrap();
            let server = Server {
                receiver,
                broadcaster: socket.broadcaster(),
                task,
            };
            let socket = socket
                .bind(config.address)
                .map_err(|_| Error::port_in_use())?;
            spawn(move || socket.run());
            let server: socket::Server = Box::new(server);
            Ok(server)
        })
    }
}

struct ConnectionFactory {
    sender: Sender<Connection>,
    task: Arc<RwLock<AtomicTask>>,
}

impl ConnectionFactory {
    fn new(sender: Sender<Connection>, task: Arc<RwLock<AtomicTask>>) -> ConnectionFactory {
        ConnectionFactory { sender, task }
    }
}

impl Factory for ConnectionFactory {
    type Handler = ConnectionHandler;
    fn connection_made(&mut self, ws: ws::Sender) -> Self::Handler {
        let (conn, handler) = Connection::new(ws);
        self.sender.send(conn).unwrap();
        self.task.read().unwrap().notify();
        handler
    }
}

impl Stream for Server {
    type Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.receiver.try_recv() {
            Ok(connection) => Ok(Async::Ready(Some(Box::new(connection)))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Server connection channel disconnected!"),
                TryRecvError::Empty => {
                    self.task.read().unwrap().register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

pub(crate) fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
    Server::listen(config)
}
