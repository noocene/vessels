use crate::{errors::Error, targets};

use super::DataChannel;

use futures::{future::err, Future, Sink, Stream};
use serde::{Deserialize, Serialize};

/// A peer-to-peer channel.
pub enum Channel {
    /// A bidirectional ordered binary data channel.
    DataChannel(Box<dyn DataChannel>),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SessionDescriptionType {
    Offer,
    Answer,
    Rollback,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NegotiationItem {
    SessionDescription(SessionDescriptionType, String),
    ConnectivityEstablishmentCandidate(Option<String>),
}

pub trait Negotiation:
    Sink<SinkItem = NegotiationItem, SinkError = Error>
    + Stream<Item = NegotiationItem, Error = Error>
    + Send
{
}

/// The remote end of a peer-to-peer network connection.
pub trait Peer: Stream<Item = Channel, Error = Error> + Send {
    /// Creates a new data channel.
    fn data_channel(
        &mut self,
    ) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error> + Send>;
}

impl dyn Peer {
    pub fn new(
    ) -> impl Future<Item = (Box<dyn Peer + 'static>, Box<dyn Negotiation + 'static>), Error = Error>
                 + Send {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::network::mesh::new();
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        targets::native::network::mesh::new()
    }
}
