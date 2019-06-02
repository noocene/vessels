use std::sync::{
    mpsc::{channel, Receiver, Sender, TryRecvError},
    Arc, RwLock,
};
use std::thread::spawn;

use futures::{lazy, Async, Future, Poll, Stream};

use ws::{CloseCode, Handler, Handshake};

use crate::errors::Error;
use crate::util::BoxedFuture;

use crate::network::{
    self,
    centralized::socket::{self, ListenConfig},
    ConnectionStatus,
};

#[derive(Clone)]
pub(crate) struct Connection {
    state: Arc<RwLock<ConnectionState>>,
}

pub(crate) struct ConnectionState {
    status: ConnectionStatus,
    details: socket::ConnectionDetails,
    sender: ws::Sender,
}

impl Connection {
    fn new(sender: ws::Sender) -> Connection {
        Connection {
            state: Arc::new(RwLock::new(ConnectionState {
                sender,
                status: ConnectionStatus::default(),
                details: socket::ConnectionDetails::default(),
            })),
        }
    }
}

impl Handler for Connection {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        self.state.write().unwrap().status = ConnectionStatus::Open;
        Ok(())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.state.write().unwrap().status = ConnectionStatus::Closed;
    }
}

pub struct OnOpen {
    context: Connection,
}

impl Future for OnOpen {
    type Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            if self.context.state.read().unwrap().status == ConnectionStatus::Open {
                return Ok(Async::Ready(Box::new(self.context.clone())));
            }
        }
    }
}

pub struct OnClose {
    context: Connection,
}

impl Future for OnClose {
    type Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            if self.context.state.read().unwrap().status == ConnectionStatus::Closed {
                return Ok(Async::Ready(Box::new(self.context.clone())));
            }
        }
    }
}

impl network::Connection for Connection {
    type TransportDetails = socket::ConnectionDetails;
    fn transport_details(&self) -> Self::TransportDetails {
        self.state.read().unwrap().details.clone()
    }
    fn status(&self) -> ConnectionStatus {
        self.state.read().unwrap().status
    }
    fn on_open(
        &self,
    ) -> Box<
        dyn Future<
                Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>,
                Error = (),
            > + Send,
    > {
        Box::new(OnOpen {
            context: self.clone(),
        })
    }
    fn on_close(
        &self,
    ) -> Box<
        dyn Future<
                Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>,
                Error = (),
            > + Send,
    > {
        Box::new(OnClose {
            context: self.clone(),
        })
    }
}

pub(crate) struct Server {
    receiver: Receiver<Connection>,
}

impl Server {
    fn create() -> (Server, Sender<Connection>) {
        let (sender, receiver) = channel();
        (Server { receiver }, sender)
    }
    fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
        lazy(move || {
            let (server, sender) = Server::create();
            let server: socket::Server = Box::new(server);
            spawn(move || {
                ws::listen(&format!("{}:{}", config.address, config.port), |out| {
                    let conn = Connection::new(out);
                    sender
                        .send(conn.clone())
                        .expect("Server connection channel disconnected!");
                    conn
                });
            });
            Ok(server)
        })
    }
}

impl Stream for Server {
    type Item = Box<dyn network::Connection<TransportDetails = socket::ConnectionDetails>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.receiver.try_recv() {
                Ok(connection) => return Ok(Async::Ready(Some(Box::new(connection)))),
                Err(err) => {
                    if let TryRecvError::Disconnected = err {
                        panic!("Server connection channel disconnected!")
                    }
                }
            }
        }
    }
}

pub(crate) fn listen(config: ListenConfig) -> impl Future<Item = socket::Server, Error = Error> {
    Server::listen(config)
}
