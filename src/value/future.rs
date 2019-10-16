use crate::{
    channel::{Channel, Fork, ForkHandle},
    value, ErasedDeserialize, SerdeAny, Value,
};

use serde::{Deserialize, Serialize};

use futures::Future as IFuture;

use std::ops::Deref;

use failure::Error;

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum FResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

pub struct Future<T, E>(Box<dyn IFuture<Item = T, Error = E> + Send + 'static>)
where
    T: Value,
    E: Value;

impl<T: Value, E: Value> Deref for Future<T, E> {
    type Target = Box<dyn IFuture<Item = T, Error = E> + Send + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> From<F> for Future<F::Item, F::Error>
where
    F: IFuture + Send + 'static,
    F::Error: Value,
    F::Item: Value,
{
    fn from(input: F) -> Self {
        Future(Box::new(input))
    }
}

#[value]
impl<T, E> Value for Future<T, E>
where
    T: Value,
    E: Value,
{
    type ConstructItem = FResult;
    type DeconstructItem = ();
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Box<dyn IFuture<Item = (), Error = ()> + Send + 'static> {
        Box::new(self.0.then(|v| {
            let fork_factory = channel.split_factory();
            channel
                .send(match v {
                    Ok(v) => FResult::Ok(fork_factory.fork(v)),
                    Err(v) => FResult::Err(fork_factory.fork(v)),
                })
                .then(|_| Ok(()))
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Box<dyn IFuture<Item = Self, Error = Error> + Send + 'static>
    where
        Self: Sized,
    {
        Box::new(channel.into_future().then(|v| match v {
            Ok(v) => Ok(match v.0.unwrap() {
                FResult::Ok(r) => Future::<T, E>::from(v.1.get_fork::<T>(r).map_err(|_| panic!())),
                FResult::Err(r) => {
                    Future::<T, E>::from(v.1.get_fork::<E>(r).then(|v| Err(v.unwrap())))
                }
            }),
            _ => panic!("lol"),
        }))
    }
}
