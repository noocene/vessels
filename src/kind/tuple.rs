use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

use void::Void;

impl<T> Kind for (T,)
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
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
            channel
                .send(channel.fork::<T>(self.0).await.unwrap())
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.unwrap();
            Ok(channel.get_fork(handle).await.unwrap())
        })
    }
}

macro_rules! tuple_impl {
    ($($len:expr => ($($n:tt $name:ident $nn:ident)+))+) => {$(
        impl<$($name),+> Kind for ($($name),+)
            where $($name: Kind),+
        {
            type ConstructItem = Vec<ForkHandle>;
            type ConstructError = Void;
            type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = Void;
            type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
            fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
                self,
                mut channel: C,
            ) -> Self::DeconstructFuture {
                Box::pin(async move {
                    channel.send(
                        vec![
                            $(channel.fork::<$name>(self.$n).await.unwrap()),+
                        ]
                    ).await.map_err(|_| panic!())
                })
            }
            fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
                mut channel: C,
            ) -> Self::ConstructFuture {
                Box::pin(async move {
                    let mut handles = channel.next().await.unwrap();
                    handles.reverse();
                    Ok(($(channel.get_fork::<$name>(handles.pop().unwrap()).await.unwrap()),+))
                })
            }
        })+
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
