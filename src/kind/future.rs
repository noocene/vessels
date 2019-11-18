use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

impl<T> Kind for BoxFuture<'static, T>
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
                .send(channel.fork(self.await).await?)
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(Box::pin(async move {
                let handle = channel.next().await.unwrap();
                channel.get_fork::<T>(handle).await.unwrap()
            }) as BoxFuture<'static, T>)
        })
    }
}
