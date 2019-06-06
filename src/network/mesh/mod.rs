use crate::{errors::Error, targets};

use super::DataChannel;

use futures::{future::err, Future, Sink, Stream};
use serde::{Deserialize, Serialize};

/// A peer-to-peer session initialization offer.
pub type Offer = String;

/// A peer-to-peer session initialization response.
pub type Answer = String;

/// A peer-to-peer channel.
pub enum PeerChannel {
    /// A bidirectional ordered binary data channel.
    DataChannel(Box<dyn DataChannel>),
}

/// A peer-to-peer networking negotiation candidate.
#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    candidate: Option<String>,
    username_fragment: String,
}

pub trait Negotiation:
    Future<Item = Box<dyn Peer>, Error = Error>
    + Sink<SinkItem = Candidate, SinkError = Error>
    + Stream<Item = Candidate, Error = Error>
    + Send
{
}

/// The remote end of a peer-to-peer network connection.
pub trait Peer: Stream<Item = PeerChannel, Error = Error> {
    /// Creates a new data channel.
    fn data_channel(&mut self) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error>>;
}

/// Creates a new peer to peer session offer.
#[allow(clippy::type_complexity)]
pub fn offer() -> impl Future<
    Item = (
        Offer,
        Box<dyn FnOnce(Answer) -> Box<dyn Negotiation + 'static> + Send + 'static>,
    ),
    Error = Error,
> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::network::mesh::offer();
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    err(Error::connection_failed())
}

/// Accepts the provided peer to peer session offer and creates an answer.
pub fn answer(
    offer: Offer,
) -> impl Future<Item = (Answer, Box<dyn Negotiation + 'static>), Error = Error> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::network::mesh::answer(offer);
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    err(Error::connection_failed())
}
