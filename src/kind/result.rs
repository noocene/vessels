use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

impl<T, E> Kind for Result<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = Result<ForkHandle, ForkHandle>;
    type ConstructError = ();
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = ();
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(match self {
                    Ok(item) => Ok(channel.fork(item).await),
                    Err(item) => Err(channel.fork(item).await),
                })
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(match channel.next().await.unwrap() {
                Ok(item) => Ok(channel.get_fork(item).await.unwrap()),
                Err(item) => Err(channel.get_fork(item).await.unwrap()),
            })
        })
    }
}
