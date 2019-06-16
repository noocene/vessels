use crate::{errors::Error, targets};

use super::DataChannel;

use std::fmt::{self, Debug, Formatter};

use futures::{Future, Sink, Stream};
use serde::{Deserialize, Serialize};

/// A peer-to-peer channel.
pub enum Channel {
    /// A bidirectional ordered binary data channel.
    DataChannel(Box<dyn DataChannel>),
}

impl Debug for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Channel ({})",
            match self {
                Channel::DataChannel(_) => "DataChannel",
            }
        )
    }
}

/// The semantic meaning of a provided session description.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SessionDescriptionType {
    /// A session offer that expects a response.
    Offer,
    /// The answer sent in response to a session offer.
    Answer,
    /// A session description intended to cause a rollback to a stable connection state.
    Rollback,
}

/// An item used in the session negotiation process.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NegotiationItem {
    /// A session description containing information on media, addresses, capabilities, etc.
    SessionDescription(SessionDescriptionType, String),
    /// A connectivity establishment candidate used in the interactive information gathering process for initial connection and novel stream creation.
    ConnectivityEstablishmentCandidate(Option<String>),
}

/// An active peer-to-peer session negotiation.
pub trait Negotiation:
    Sink<SinkItem = NegotiationItem, SinkError = Error>
    + Stream<Item = NegotiationItem, Error = Error>
    + Send
{
}

/// A peer-to-peer network coordinator connected to a single remote peer.
pub trait Peer: Stream<Item = Channel, Error = Error> + Send {
    /// Creates a new data channel.
    fn data_channel(
        &mut self,
    ) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error> + Send>;
}

impl dyn Peer {
    /// Creates a new peer and negotiation pair.
    pub fn new(
    ) -> impl Future<Item = (Box<dyn Peer + 'static>, Box<dyn Negotiation + 'static>), Error = Error>
                 + Send {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::network::mesh::new();
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        targets::native::network::mesh::new()
    }
}
