use crate::{
    channel::{Channel, Fork, ForkHandle},
    value, Entity, ErasedDeserialize, SerdeAny,
};

use serde::{Deserialize, Serialize};

use futures::Future;

#[doc(hidden)]
#[derive(Serialize, Deserialize)]
pub enum VResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

#[value]
impl<T, E> Entity for Result<T, E>
where
    T: Entity,
    E: Entity,
{
    type ConstructItem = VResult;
    type ConstructFuture = Box<dyn Future<Item = Self, Error = ()> + Send>;
    type DeconstructItem = ();
    type DeconstructFuture = Box<dyn Future<Item = (), Error = ()> + Send>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        match self {
            Ok(v) => Box::new(
                channel
                    .fork(v)
                    .and_then(|h| channel.send(VResult::Ok(h)).then(|_| Ok(()))),
            ),
            Err(v) => Box::new(
                channel
                    .fork(v)
                    .and_then(|h| channel.send(VResult::Err(h)).then(|_| Ok(()))),
            ),
        }
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(channel.into_future().then(|v| {
            match v {
                Ok(v) => match v.0.unwrap() {
                    VResult::Ok(r) => Box::new(
                        v.1.get_fork::<T>(r)
                            .map(|item| Result::<T, E>::Ok(item))
                            .map_err(|_| panic!()),
                    )
                        as Box<dyn Future<Item = Result<T, E>, Error = ()> + Send>,
                    VResult::Err(r) => Box::new(v.1.get_fork::<E>(r).then(|item| {
                        Ok(if let Ok(e) = item {
                            Result::<T, E>::Err(e)
                        } else {
                            panic!()
                        })
                    }))
                        as Box<dyn Future<Item = Result<T, E>, Error = ()> + Send>,
                },
                _ => panic!("lol"),
            }
        }))
    }
}
