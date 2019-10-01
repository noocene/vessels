use futures::Stream;
use serde::{de::DeserializeOwned, Serialize};

/// A generated remote binding of a trait created by `protocol`.
pub trait Remote: Stream<Item = <Self as Remote>::Item, Error = ()> {
    /// The associated opaque call type used by the `protocol`.
    type Item: Serialize + DeserializeOwned;
}

pub use vessels_derive::protocol;
