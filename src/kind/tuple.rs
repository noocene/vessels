use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::Future,
    ConstructResult, DeconstructResult, Kind,
};

use futures::{SinkExt, StreamExt};

use super::WrappedError;

use void::Void;

#[kind]
impl<T> Kind for (T,)
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = WrappedError<T::ConstructError>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<T::DeconstructError>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
                .send(channel.fork::<T>(self.0).await?)
                .await
                .map_err(WrappedError::Send)?)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(WrappedError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok((channel.get_fork::<T>(handle).await?,))
        })
    }
}

macro_rules! tuple_impl {
    ($($len:expr => ($($n:tt $name:ident $nn:ident)+))+) => {$(
        #[kind]
        impl<$($name),+> Kind for ($($name),+)
            where $($name: Kind),+
        {
            type ConstructItem = Vec<ForkHandle>;
            type ConstructError = WrappedError<Void>;
            type ConstructFuture = Future<ConstructResult<Self>>;
            type DeconstructItem = ();
            type DeconstructError = Void;
            type DeconstructFuture = Future<DeconstructResult<Self>>;
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
                    let item = channel.next().await.ok_or(WrappedError::Insufficient {
                        got: 0,
                        expected: 1
                    })?;
                    Ok(($(channel.get_fork::<$name>(item[$n]).await.unwrap()),+))
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
