use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{task::AtomicTask, Async, IntoFuture, Poll, Stream};
use serde::{
    de::DeserializeOwned,
    ser::{SerializeSeq, Serializer},
    Serialize,
};
use std::{marker::PhantomData, sync::Arc};

/// A generated remote binding of a trait created by `protocol`.
pub trait Remote: Stream<Item = <Self as Remote>::Item, Error = ()> {
    /// The associated opaque call type used by the `protocol`.
    type Item: Serialize + DeserializeOwned;
}

pub use vitruvia_derive::protocol;
