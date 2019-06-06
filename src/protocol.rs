use futures::Stream;
use serde::{Serialize, de::DeserializeOwned};

pub trait Remote<T>: Stream<Item = T, Error = ()> where T: Serialize + DeserializeOwned {}

pub use vitruvia_derive::protocol;