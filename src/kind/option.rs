use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

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
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(match self {
                    None => None,
                    Some(item) => Some(channel.fork(item).await),
                })
                .await
                .unwrap_or_else(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(match channel.next().await.unwrap() {
                Some(item) => Some(channel.get_fork(item).await.unwrap()),
                None => None,
            })
        })
    }
}
