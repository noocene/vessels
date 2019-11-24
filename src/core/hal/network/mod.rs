use crate::{core::UnimplementedError, kind::Future, object, Kind};

use failure::{Error, Fail};
use std::net::SocketAddr;
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

#[object]
pub trait Client<K: Kind> {
    fn connect(&mut self, address: Url) -> Future<Result<K, ConnectError>>;
}

#[object]
pub trait Server<K: Kind> {
    fn listen(
        &mut self,
        address: SocketAddr,
        handler: Box<dyn FnMut() -> Future<K> + Sync + Send>,
    ) -> Future<Result<(), ListenError>>;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl<K: Kind> dyn Client<K> {
    pub fn new() -> Result<Box<dyn Client<K>>, UnimplementedError> {
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

impl<K: Kind> dyn Server<K> {
    pub fn new() -> Result<Box<dyn Server<K>>, UnimplementedError> {
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
