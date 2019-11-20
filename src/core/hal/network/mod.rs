use crate::{
    core::UnimplementedError,
    kind::{Future, Stream},
    object, Kind,
};

use failure::{Error, Fail};
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Kind)]
pub struct StaticCandidate {
    pub ufrag: [u8; 3],
    pub pwd: [u8; 24],
    pub fingerprint: [u8; 32],
    pub addr: SocketAddr,
}

#[object]
pub trait Peer {}

#[derive(Fail, Debug, Kind)]
#[fail(display = "connection failed")]
pub struct ConnectError;

#[derive(Fail, Debug, Kind)]
#[fail(display = "listening failed: {}", cause)]
pub struct ListenError {
    #[fail(cause)]
    cause: Error,
}

#[object]
pub trait Client {
    fn connect(&mut self, address: StaticCandidate) -> Future<Result<Box<dyn Peer>, ConnectError>>;
}

#[object]
pub trait Server {
    fn listen(&mut self, address: SocketAddr) -> Stream<Result<Box<dyn Peer>, ListenError>>;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn Client {
    pub fn new() -> Result<Box<dyn Client>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Client::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Err(UnimplementedError {
            feature: "a network client".to_owned(),
        });
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "a network client".to_owned(),
        });
    }
}

impl dyn Server {
    pub fn new() -> Result<Box<dyn Server>, UnimplementedError> {
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
