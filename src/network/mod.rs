use crate::errors::Error;
use futures::{Sink, Stream};

/// Provides peer-to-peer mesh networking functionality.
pub mod mesh;

/// Provides client/server networking functionality.
pub mod centralized;

/// A bidirectional binary data channel.
pub trait DataChannel:
    Stream<Item = Vec<u8>, Error = Error> + Sink<SinkItem = Vec<u8>, SinkError = Error> + Send
{
}
