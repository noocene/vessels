use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{
    future::BoxFuture,
    stream::{unfold, BoxStream},
    SinkExt, StreamExt,
};

impl<T> Kind for BoxStream<'static, T>
where
    T: Kind,
{
    type ConstructItem = Option<ForkHandle>;
    type ConstructError = T::ConstructError;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = T::DeconstructError;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(item) = self.next().await {
                channel
                    .send(Some(channel.fork(item).await.unwrap()))
                    .await
                    .map_err(|_| panic!())
                    .unwrap()
            }
            channel.send(None).await.map_err(|_| panic!()).unwrap();
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(Box::pin(unfold(channel, |mut channel| {
                async move {
                    if let Some(handle) = channel.next().await.unwrap() {
                        Some((channel.get_fork(handle).await.unwrap(), channel))
                    } else {
                        None
                    }
                }
            })) as BoxStream<'static, T>)
        })
    }
}
