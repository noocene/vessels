use crate::{
    channel::{Channel, ForkHandle},
    Kind,
};

use futures::{future::join_all, Future};

impl<T> Kind for Vec<T>
where
    T: Kind,
{
    type ConstructItem = Vec<ForkHandle>;
    type ConstructFuture = Box<dyn Future<Item = Self, Error = ()> + Send>;
    type DeconstructItem = ();
    type DeconstructFuture = Box<dyn Future<Item = (), Error = ()> + Send>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        Box::new(
            join_all(
                self.into_iter()
                    .map(|entry| channel.fork::<T>(entry))
                    .collect::<Vec<Box<dyn Future<Item = ForkHandle, Error = ()> + Send>>>(),
            )
            .map_err(|_| panic!("lol"))
            .and_then(|handles| channel.send(handles).then(|_| Ok(()))),
        )
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::new(
            channel
                .into_future()
                .map_err(|_| panic!("lol"))
                .and_then(|(item, channel)| {
                    join_all(
                        item.unwrap()
                            .into_iter()
                            .map(|entry| channel.get_fork::<T>(entry))
                            .collect::<Vec<Box<dyn Future<Item = T, Error = ()> + Send>>>(),
                    )
                }),
        )
    }
}
