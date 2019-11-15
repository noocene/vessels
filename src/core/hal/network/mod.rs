use crate::{kind::Future, object, Kind};

use std::net::SocketAddr;

#[derive(Kind)]
pub struct StaticCandidate {
    ufrag: [u8; 3],
    pwd: [u8; 16],
    fingerprint: [u8; 32],
    addr: SocketAddr,
}

#[object]
pub trait Peer {}

#[object]
pub trait Network {
    fn connect(&self, address: StaticCandidate) -> Future<Box<dyn Peer>>;
}
