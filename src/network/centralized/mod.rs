use super::Connection;
use crate::errors::Error;
use futures::Stream;

pub mod socket;

pub type Server<T> =
    Box<dyn Stream<Item = Box<dyn Connection<TransportDetails = T>>, Error = Error> + Send>;
