use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

use futures::{
    future::{try_join_all, BoxFuture},
    SinkExt, StreamExt, TryFutureExt,
};

macro_rules! iterator_impl {
    ($($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >),+) => {$(
        impl<T $(, $typaram)*> Kind for $ty<T $(, $typaram)*>
            where T: Kind $(+ $tbound1 $(+ $tbound2)*)*, $($typaram: $bound,)*
        {
            type ConstructItem = Vec<ForkHandle>;
            type ConstructError = T::ConstructError;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = T::DeconstructError;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    channel.send(try_join_all(
                        self.into_iter()
                            .map(|entry| channel.fork::<T>(entry)),
                    ).await?).await.map_err(|_| panic!())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    try_join_all(
                        handles
                            .into_iter()
                            .map(|entry| channel.get_fork::<T>(entry)),
                    )
                    .map_ok(|vec| vec.into_iter().collect()).await
                })
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
            type ConstructError = <(K, V) as Kind>::ConstructError;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = <(K, V) as Kind>::DeconstructError;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    channel.send(try_join_all(
                        self.into_iter()
                            .map(|entry| channel.fork::<(K, V)>(entry))
                    ).await?).await.map_err(|_| panic!())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let handles = channel.next().await.unwrap();
                    try_join_all(
                        handles
                            .into_iter()
                            .map(|entry| channel.get_fork::<(K, V)>(entry)),
                    )
                    .map_ok(|vec| vec.into_iter().collect()).await
                })
            }
        }
    )+};
}

map_impl! {
    BTreeMap<K: Ord, V>,
    HashMap<K: Eq + Hash, V>
}
