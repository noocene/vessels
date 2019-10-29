use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use futures::{
    future::{ok, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

impl<T, E> Kind for Result<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = Result<ForkHandle, ForkHandle>;
    type Error = ();
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructFuture = BoxFuture<'static, ()>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        match self {
            Ok(v) => Box::pin(channel.fork(v).map(Ok).then(|handle| {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(
                    once(ok(handle))
                        .forward(channel)
                        .unwrap_or_else(|_| panic!()),
                )
            })),
            Err(v) => Box::pin(channel.fork(v).map(Err).then(|handle| {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(
                    once(ok(handle))
                        .forward(channel)
                        .unwrap_or_else(|_| panic!()),
                )
            })),
        }
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(channel.into_future().then(move |(item, channel)| {
            item.unwrap()
                .map(|handle| {
                    Box::pin(
                        channel
                            .get_fork::<T>(handle)
                            .map_ok(Ok)
                            .map_err(|_| panic!()),
                    ) as BoxFuture<'static, ConstructResult<Self>>
                })
                .unwrap_or_else(|handle| {
                    Box::pin(
                        channel
                            .get_fork::<E>(handle)
                            .map_ok(Err)
                            .map_err(|_| panic!()),
                    ) as BoxFuture<'static, ConstructResult<Self>>
                })
        }))
    }
}
