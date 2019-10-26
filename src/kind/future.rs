use crate::{
    channel::{Channel, Fork, ForkHandle},
    kind, ErasedDeserialize, Kind, SerdeAny,
};

use serde::{Deserialize, Serialize};

use futures::Future as IFuture;

use failure::Error;

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum VResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

pub type Future<T, E> = Box<dyn IFuture<Item = T, Error = E> + Send>;

#[kind]
impl<T, E> Kind for Box<dyn IFuture<Item = T, Error = E> + Send>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = VResult;
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
                        .and_then(|h| channel.send(VResult::Ok(h)).then(|_| Ok(()))),
                ),
                Err(v) => Box::new(
                    fork_factory
                        .fork(v)
                        .and_then(|h| channel.send(VResult::Err(h)).then(|_| Ok(()))),
                ) as Box<dyn IFuture<Item = (), Error = ()> + Send>,
            }
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(channel.into_future().then(|v| match v {
            Ok(v) => Ok(match v.0.unwrap() {
                VResult::Ok(r) => Box::new(v.1.get_fork::<T>(r).map_err(|_| -> E { panic!() }))
                    as Box<dyn IFuture<Item = T, Error = E> + Send>,
                VResult::Err(r) => Box::new(v.1.get_fork::<E>(r).then(|v| Err(v.unwrap())))
                    as Box<dyn IFuture<Item = T, Error = E> + Send>,
            }),
            _ => panic!("lol"),
        }))
    }
}
