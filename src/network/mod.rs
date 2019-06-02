use crate::errors::Error;
use futures::Stream;

/// Provides peer-to-peer mesh networking functionality.
pub mod mesh;

/// Provides client/server networking functionality.
pub mod centralized;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum ConnectionStatus {
    Connecting,
    Open,
    Closed,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        ConnectionStatus::Connecting
    }
}

pub trait Connection: Stream<Item = Vec<u8>, Error = Error> + Send {
    type TransportDetails;
    fn transport_details(&self) -> &Self::TransportDetails;
    fn status(&self) -> ConnectionStatus;
}
