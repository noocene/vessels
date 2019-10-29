use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use futures::{
    future::ok, future::BoxFuture, stream::once, FutureExt, SinkExt, StreamExt, TryFutureExt,
};

impl<T> Kind for Option<T>
where
    T: Kind,
{
    type ConstructItem = Option<ForkHandle>;
    type Error = T::Error;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructFuture = BoxFuture<'static, ()>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        match self {
            Some(v) => Box::pin(channel.fork(v).map(Some).then(|handle| {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(
                    once(ok(handle))
                        .forward(channel)
                        .unwrap_or_else(|_| panic!()),
                )
            })),
            None => {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(once(ok(None)).forward(channel).unwrap_or_else(|_| panic!()))
            }
        }
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(channel.into_future().then(move |(item, channel)| {
            item.unwrap().map_or_else(
                || Box::pin(ok(None)) as BoxFuture<'static, ConstructResult<Self>>,
                move |handle| Box::pin(channel.get_fork(handle).map_ok(Some)),
            )
        }))
    }
}
