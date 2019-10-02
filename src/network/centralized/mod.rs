use super::DataChannel;
use futures::Stream;
use failure::Error;

/// Provides an abstracted client-server model binary line-codec socket.
pub mod socket;

/// A server that provides a stream of connections.
pub type Server = Box<dyn Stream<Item = Box<dyn DataChannel>, Error = Error>>;
