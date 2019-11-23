use crate::{
    channel::{Channel, ForkHandle},
    kind::{Future, Stream},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{stream::unfold, SinkExt, StreamExt};

use super::WrappedError;

impl<T> Kind for Stream<T>
where
    T: Kind,
{
    type ConstructItem = Option<ForkHandle>;
    type ConstructError = WrappedError<T::ConstructError>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<T::DeconstructError>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(item) = self.next().await {
                channel.send(Some(channel.fork(item).await?)).await?
            }
            channel.send(None).await?;
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
            })) as Stream<T>)
        })
    }
}
