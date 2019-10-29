use futures::try_join;

use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use futures::{
    future::{join_all, ok, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

macro_rules! tuple_impl {
    ($($len:expr => ($($n:tt $name:ident $nn:ident)+))+) => {$(
        impl<$($name),+> Kind for ($($name),+)
where
    $($name: Kind),+ // each element must be Kind
{
            type ConstructItem = Vec<ForkHandle>;
            type Error = ();
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructFuture = BoxFuture<'static, ()>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(
                    join_all(
                        vec![
                            $(channel.fork::<$name>(self.$n)),+
                        ]
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
                        .into_future()
                        .then(move |(item, channel)| {
                            let item = item.unwrap();
                            let ($($nn),+) = ($(channel.get_fork::<$name>(item[$n]).map_err(|_| panic!("lol"))),+);
                            async {
                                Ok(try_join!($($nn),+).unwrap_or_else(|_| panic!("lol")))
                            }
                        })
                )
            }
        })+
    }
}

impl<T> Kind for (T,)
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type Error = T::Error;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructFuture = BoxFuture<'static, ()>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(channel.fork::<T>(self.0).then(move |handles| {
            let channel = channel.sink_map_err(|_| panic!());
            Box::pin(
                once(ok(handles))
                    .forward(channel)
                    .unwrap_or_else(|_| panic!()),
            )
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(channel.into_future().then(move |(item, channel)| {
            let item = item.unwrap();
            channel.get_fork::<T>(item).map_ok(|t| (t,))
        }))
    }
}
tuple_impl! {
    2 => (0 T0 a 1 T1 b)
    3 => (0 T0 a 1 T1 b 2 T2 c)
    4 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d)
    5 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e)
    6 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f)
    7 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g)
    8 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h)
    9 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i)
    10 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j)
    11 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k)
    12 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l)
    13 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m)
    14 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n)
    15 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o)
    16 => (0 T0 a 1 T1 b 2 T2 c 3 T3 d 4 T4 e 5 T5 f 6 T6 g 7 T7 h 8 T8 i 9 T9 j 10 T10 k 11 T11 l 12 T12 m 13 T13 n 14 T14 o 15 T15 p)
}
