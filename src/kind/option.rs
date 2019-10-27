use crate::{
    channel::{Channel, ForkHandle},
    kind, ErasedDeserialize, Kind, SerdeAny,
};

use serde::{Deserialize, Serialize};

use futures::{future::ok, Future};

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub enum VOption {
    Some(ForkHandle),
    None,
}

#[kind]
impl<T> Kind for Option<T>
where
    T: Kind,
{
    type ConstructItem = VOption;
    type ConstructFuture = Box<dyn Future<Item = Self, Error = ()> + Send>;
    type DeconstructItem = ();
    type DeconstructFuture = Box<dyn Future<Item = (), Error = ()> + Send>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        match self {
            Some(v) => Box::new(
                channel
                    .fork(v)
                    .and_then(|h| channel.send(VOption::Some(h)).then(|_| Ok(()))),
            ),
            None => Box::new(channel.send(VOption::None).then(|_| Ok(())))
                as Box<dyn Future<Item = (), Error = ()> + Send>,
        }
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(channel.into_future().then(|v| match v {
            Ok(v) => match v.0.unwrap() {
                VOption::Some(r) => Box::new(v.1.get_fork::<T>(r).map(Some).map_err(|_| panic!()))
                    as Box<dyn Future<Item = Option<T>, Error = ()> + Send>,
                VOption::None => {
                    Box::new(ok(None)) as Box<dyn Future<Item = Option<T>, Error = ()> + Send>
                }
            },
            _ => panic!("lol"),
        }))
    }
}
