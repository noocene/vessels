use crate::{
    channel::{Channel, ForkHandle},
    Kind,
};
use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

use futures::{future::join_all, Future};

macro_rules! iterator_impl {
    ($($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >),+) => {$(
        impl<T $(, $typaram)*> Kind for $ty<T $(, $typaram)*>
            where T: Kind + Send + 'static $(+ $tbound1 $(+ $tbound2)*)*, $($typaram: $bound,)*
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
                    .and_then(|handles| {
                        channel
                            .send(handles)
                            .and_then(|_| Ok(()))
                            .map_err(|_| panic!())
                    }),
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
                            .map(|collection_as_vec| collection_as_vec.into_iter().collect())
                        }),
                )
            }
        }
    )+};
}

iterator_impl!(
    BinaryHeap<T: Ord>,
    BTreeSet<T: Ord>,
    HashSet<T: Hash + Eq>,
    LinkedList<T>,
    Vec<T>,
    VecDeque<T>
);
