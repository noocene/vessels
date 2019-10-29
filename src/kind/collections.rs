use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

use futures::{
    future::{join_all, ok, try_join_all, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

macro_rules! iterator_impl {
    ($($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >),+) => {$(
        impl<T $(, $typaram)*> Kind for $ty<T $(, $typaram)*>
            where T: Kind $(+ $tbound1 $(+ $tbound2)*)*, $($typaram: $bound,)*
        {
            type ConstructItem = Vec<ForkHandle>;
            type Error = T::Error;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructFuture = BoxFuture<'static, ()>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(
                    join_all(
                        self.into_iter()
                            .map(|entry| channel.fork::<T>(entry)),
                    )
                    .then(move |handles| {
                        let channel = channel.sink_map_err(|_| panic!());
                        Box::pin(
                            once(ok(handles))
                                .forward(channel)
                                .unwrap_or_else(|_| panic!()),
                        )
                    }),
                )
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(
                    channel
                        .into_future().then(move |(item, channel)| {
                            try_join_all(
                                item.unwrap()
                                    .into_iter()
                                    .map(|entry| channel.get_fork::<T>(entry)),
                            )
                            .map_ok(|vec| vec.into_iter().collect())
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

macro_rules! map_impl {
    ($($ty:ident < K $(: $tbound1:ident $(+ $tbound2:ident)*)*, V >),+) => {$(
        impl<K, V> Kind for $ty<K, V>
            where K: Kind $(+ $tbound1 $(+ $tbound2)*)*,
            V: Kind
        {
            type ConstructItem = Vec<ForkHandle>;
            type Error = <(K, V) as Kind>::Error;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructFuture = BoxFuture<'static, ()>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(
                    join_all(
                        self.into_iter()
                            .map(|entry| channel.fork::<(K, V)>(entry)),
                    )
                    .then(move |handles| {
                        let channel = channel.sink_map_err(|_| panic!());
                        Box::pin(
                            once(ok(handles))
                                .forward(channel)
                                .unwrap_or_else(|_| panic!()),
                        )
                    }),
                )
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(
                    channel
                        .into_future().then(move |(item, channel)| {
                            try_join_all(
                                item.unwrap()
                                    .into_iter()
                                    .map(|entry| channel.get_fork::<(K, V)>(entry)),
                            )
                            .map_ok(|vec| vec.into_iter().collect())
                        }),
                )
            }
        }
    )+};
}

map_impl! {
    BTreeMap<K: Ord, V>,
    HashMap<K: Eq + Hash, V>
}
