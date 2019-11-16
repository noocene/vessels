use crate::{core::UnimplementedError, kind::Future, object, Kind};

use failure::Fail;
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Kind)]
pub struct StaticCandidate {
    pub ufrag: [u8; 3],
    pub pwd: [u8; 16],
    pub fingerprint: [u8; 32],
    pub addr: SocketAddr,
}

#[object]
pub trait Peer {}

#[derive(Fail, Debug, Kind)]
#[fail(display = "connection failed")]
pub struct ConnectError;

#[object]
pub trait Network {
    fn connect(&mut self, address: StaticCandidate) -> Future<Result<Box<dyn Peer>, ConnectError>>;
}

#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn Network {
    pub fn new() -> Result<Box<dyn Network>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Network::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Rng::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "networking".to_owned(),
        });
    }
}
