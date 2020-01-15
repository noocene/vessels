use crate::{
    channel::{Context, OnTo, Target},
    core::{spawn, UnimplementedError},
    format::{ApplyDecode, ApplyEncode, Format},
    kind::{Fallible, Future, Infallible, SinkStream, TransportError},
    object, Kind,
};

use anyhow::Error;
use futures::{future::ready, lock::Mutex, FutureExt, Sink, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use url::Url;

#[object]
pub trait Peer {}

#[derive(Error, Debug, Kind)]
pub enum ConnectError {
    #[error("connection failed: `{0}`")]
    Connect(#[source] Error),
    #[error("construct failed: `{0}`")]
    Construct(#[source] Error),
    #[error("underlying transport failed: `{0}`")]
    Transport(#[from] TransportError),
}

#[derive(Error, Debug, Kind)]
#[error("listening failed: {cause}")]
pub struct ListenError {
    #[source]
    cause: Error,
}

impl From<TransportError> for ListenError {
    fn from(error: TransportError) -> Self {
        ListenError {
            cause: error.into(),
        }
    }
}

#[derive(Error, Debug, Kind)]
#[error("connection failed while open: {cause}")]
pub struct ConnectionError {
    #[source]
    cause: Error,
}

impl From<TransportError> for ConnectionError {
    fn from(error: TransportError) -> Self {
        ConnectionError { cause: error.into() }
    }
}

#[object]
pub(crate) trait RawClient {
    fn connect(
        &mut self,
        address: Url,
    ) -> Fallible<SinkStream<Vec<u8>, ConnectionError, Vec<u8>>, ConnectError>;
}

#[derive(Kind)]
pub struct Client(Box<dyn RawClient>);

impl Client {
    pub fn new() -> Result<Client, UnimplementedError> {
        RawClient::new().map(Client)
    }
    pub fn connect<
        'a,
        K: Kind,
        T: Target<'a, K> + 'static,
        F: Format<Representation = Vec<u8>> + 'static,
    >(
        &mut self,
        address: Url,
    ) -> Fallible<K, ConnectError> {
        let connection = self.0.connect(address);
        Box::pin(async move {
            connection
                .await?
                .decode::<T, F>()
                .await
                .map_err(|e| ConnectError::Construct(e.into()))
        })
    }
}

#[object]
pub(crate) trait RawServer {
    fn listen(
        &mut self,
        address: SocketAddr,
        handler: Box<
            dyn FnMut(SinkStream<Vec<u8>, ConnectionError, Vec<u8>>) -> Infallible<()>
                + Sync
                + Send,
        >,
    ) -> Fallible<(), ListenError>;
}

#[derive(Kind)]
pub struct Server(Box<dyn RawServer>);

impl Server {
    pub fn new() -> Result<Server, UnimplementedError> {
        RawServer::new().map(Server)
    }
    pub fn listen<
        'a,
        K: Kind,
        T: Target<'a, K> + 'static,
        F: Format<Representation = Vec<u8>> + 'static,
    >(
        &mut self,
        address: SocketAddr,
        handler: Box<dyn FnMut() -> Future<K> + Sync + Send>,
    ) -> Fallible<(), ListenError>
    where
        T: ApplyEncode<'a>,
        <T as Sink<<T as Context<'a>>::Item>>::Error: std::error::Error + Sync + Send + 'static,
    {
        let handler = Arc::new(Mutex::new(handler));
        self.0.listen(
            address,
            Box::new(move |channel| {
                let handler = handler.clone();
                Box::pin(async move {
                    let (sender, receiver) = channel.split();
                    let (sink, stream) = (handler.lock().await.as_mut())()
                        .await
                        .on_to::<T>()
                        .await
                        .encode::<F>()
                        .split();
                    spawn(stream.map(Ok).forward(sender).then(|_| ready(())));
                    spawn(receiver.map(Ok).forward(sink).then(|_| ready(())));
                    Ok(())
                })
            }),
        )
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn RawClient {
    fn new() -> Result<Box<dyn RawClient>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Client::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Client::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "a network client".to_owned(),
        });
    }
}

impl dyn RawServer {
    fn new() -> Result<Box<dyn RawServer>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Err(UnimplementedError {
            feature: "a network server".to_owned(),
        });
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Server::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "a network server".to_owned(),
        });
    }
}
