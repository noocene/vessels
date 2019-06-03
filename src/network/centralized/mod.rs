use super::DataChannel;
use crate::errors::Error;
use futures::Stream;

/// Provides an abstracted client-server model binary line-codec socket.
pub mod socket;

/// A server that provides a [futures::Stream] of connections.
pub type Server = Box<dyn Stream<Item = Box<dyn DataChannel>, Error = Error> + Send>;
