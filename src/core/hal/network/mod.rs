use crate::{
    channel::{Context, OnTo, Target},
    core,
    core::{executor::Spawn, Executor, UnimplementedError},
    format::{ApplyDecode, ApplyEncode, Format},
    kind::{Future, SinkStream},
    object, Kind,
};

use failure::{Error, Fail};
use futures::{future::ready, lock::Mutex, FutureExt, Sink, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use url::Url;

#[object]
pub trait Peer {}

#[derive(Fail, Debug, Kind)]
pub enum ConnectError {
    #[fail(display = "connection failed: {}", _0)]
    Connect(#[cause] Error),
    #[fail(display = "construct failed: {}", _0)]
    Construct(#[cause] Error),
}

#[derive(Fail, Debug, Kind)]
#[fail(display = "listening failed: {}", cause)]
pub struct ListenError {
    #[fail(cause)]
    cause: Error,
}

#[derive(Fail, Debug, Kind)]
#[fail(display = "connection failed while open: {}", cause)]
pub struct ConnectionError {
    #[fail(cause)]
    cause: Error,
}

#[object]
pub(crate) trait RawClient {
    fn connect(
        &mut self,
        address: Url,
    ) -> Future<Result<SinkStream<Vec<u8>, ConnectionError, Vec<u8>>, ConnectError>>;
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
    ) -> Future<Result<K, ConnectError>> {
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
            dyn FnMut(SinkStream<Vec<u8>, ConnectionError, Vec<u8>>) -> Future<()> + Sync + Send,
        >,
    ) -> Future<Result<(), ListenError>>;
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
    ) -> Future<Result<(), ListenError>>
    where
        T: ApplyEncode<'a>,
        <T as Sink<<T as Context<'a>>::Item>>::Error: Fail,
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
                    core::<dyn Executor>()
                        .unwrap()
                        .spawn(stream.map(Ok).forward(sender).then(|_| ready(())));
                    core::<dyn Executor>()
                        .unwrap()
                        .spawn(receiver.map(Ok).forward(sink).then(|_| ready(())));
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
