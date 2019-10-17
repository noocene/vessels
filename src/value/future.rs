use crate::{
    channel::{Channel, Fork, ForkHandle},
    value, ErasedDeserialize, SerdeAny, Value,
};

use serde::{Deserialize, Serialize};

use futures::Future as IFuture;

use failure::Error;

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum FResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

pub type Future<T, E> = Box<dyn IFuture<Item = T, Error = E> + Send>;

#[value]
impl<T, E> Value for Box<dyn IFuture<Item = T, Error = E> + Send>
where
    T: Value,
    E: Value,
{
    type ConstructItem = FResult;
    type ConstructFuture = Box<dyn IFuture<Item = Self, Error = Error> + Send>;
    type DeconstructItem = ();
    type DeconstructFuture = Box<dyn IFuture<Item = (), Error = ()> + Send>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        Box::new(self.then(|v| {
            let fork_factory = channel.split_factory();
            match v {
                Ok(v) => Box::new(
                    fork_factory
                        .fork(v)
                        .and_then(|h| channel.send(FResult::Ok(h)).then(|_| Ok(()))),
                ),
                Err(v) => Box::new(
                    fork_factory
                        .fork(v)
                        .and_then(|h| channel.send(FResult::Err(h)).then(|_| Ok(()))),
                ) as Box<dyn IFuture<Item = (), Error = ()> + Send>,
            }
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(channel.into_future().then(|v| match v {
            Ok(v) => Ok(match v.0.unwrap() {
                FResult::Ok(r) => Box::new(v.1.get_fork::<T>(r).map_err(|_| -> E { panic!() }))
                    as Box<dyn IFuture<Item = T, Error = E> + Send>,
                FResult::Err(r) => Box::new(v.1.get_fork::<E>(r).then(|v| Err(v.unwrap())))
                    as Box<dyn IFuture<Item = T, Error = E> + Send>,
            }),
            _ => panic!("lol"),
        }))
    }
}
