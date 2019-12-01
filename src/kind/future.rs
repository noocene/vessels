use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::Future,
    ConstructResult, DeconstructResult, Kind,
};

use futures::{SinkExt, StreamExt};

use super::WrappedError;

#[kind]
impl<T> Kind for Future<T>
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = T::ConstructError;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<T::DeconstructError>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move { Ok(channel.send(channel.fork(self.await).await?).await?) })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(Box::pin(async move {
                let handle = channel.next().await.unwrap();
                channel.get_fork::<T>(handle).await.unwrap()
            }) as Future<T>)
        })
    }
}
