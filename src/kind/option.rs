use crate::{
    channel::{Channel, ForkHandle},
    Kind,
};

use serde::{Deserialize, Serialize};

use futures::{future::ok, Future};

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub enum KOption {
    Some(ForkHandle),
    None,
}

impl<T> Kind for Option<T>
where
    T: Kind,
{
    type ConstructItem = KOption;
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
                    .and_then(|h| channel.send(KOption::Some(h)).then(|_| Ok(()))),
            ),
            None => Box::new(channel.send(KOption::None).then(|_| Ok(())))
                as Box<dyn Future<Item = (), Error = ()> + Send>,
        }
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(
            channel
                .into_future()
                .map_err(|_| panic!("lol"))
                .and_then(|(item, channel)| match item.unwrap() {
                    KOption::Some(r) => {
                        Box::new(channel.get_fork::<T>(r).map(Some).map_err(|_| panic!()))
                            as Box<dyn Future<Item = Option<T>, Error = ()> + Send>
                    }
                    KOption::None => {
                        Box::new(ok(None)) as Box<dyn Future<Item = Option<T>, Error = ()> + Send>
                    }
                }),
        )
    }
}
