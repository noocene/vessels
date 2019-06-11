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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectivityEstablishmentCandidate {
    pub candidate: String,
    pub username_fragment: String,
    pub media_id: String,
    pub media_line_index: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NegotiationItem {
    SessionDescription(SessionDescriptionType, String),
    ConnectivityEstablishmentCandidate(Option<ConnectivityEstablishmentCandidate>),
}

#[derive(Clone, Copy, Debug)]
pub enum Role {
    Offering,
    Answering,
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
    fn data_channel(&mut self) -> Box<dyn Future<Item = Box<dyn DataChannel>, Error = Error>>;
}

impl dyn Peer {
    pub fn new(role: Role) -> (Box<dyn Peer>, Box<dyn Negotiation>) {
        targets::web::network::mesh::new(role)
    }
}
