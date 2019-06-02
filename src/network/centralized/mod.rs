use super::DataChannel;
use crate::errors::Error;
use futures::Stream;

pub mod socket;

pub type Server = Box<dyn Stream<Item = Box<dyn DataChannel>, Error = Error> + Send>;
