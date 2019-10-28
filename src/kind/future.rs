use crate::{
    channel::{Channel, Fork, ForkHandle},
    Kind,
};

use serde::{Deserialize, Serialize};

use futures::{Future as IFuture, Poll};

use failure::Error;

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub enum KResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

pub struct Future<T: Kind, E: Kind>(Box<dyn IFuture<Item = T, Error = E> + Send>);

impl<T: Kind, E: Kind> IFuture for Future<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl<T: Kind, E: Kind> Future<T, E> {
    pub fn new<F: IFuture<Item = T, Error = E> + Send + 'static>(future: F) -> Self {
        Future(Box::new(future))
    }
}

impl<T, E> Kind for Future<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = KResult;
    type ConstructFuture = Box<dyn IFuture<Item = Self, Error = ()> + Send>;
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
                        .and_then(|h| channel.send(KResult::Ok(h)).then(|_| Ok(()))),
                ),
                Err(v) => Box::new(
                    fork_factory
                        .fork(v)
                        .and_then(|h| channel.send(KResult::Err(h)).then(|_| Ok(()))),
                ) as Box<dyn IFuture<Item = (), Error = ()> + Send>,
            }
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(channel.into_future().then(|item| {
            if let Ok((result, channel)) = item {
                Ok(match result.unwrap() {
                    KResult::Ok(r) => Future(Box::new(
                        channel.get_fork::<T>(r).map_err(|_| -> E { panic!() }),
                    )
                        as Box<dyn IFuture<Item = T, Error = E> + Send>),
                    KResult::Err(r) => {
                        Future(Box::new(channel.get_fork::<E>(r).then(|v| Err(v.unwrap())))
                            as Box<dyn IFuture<Item = T, Error = E> + Send>)
                    }
                })
            } else {
                panic!("lol")
            }
        }))
    }
}
