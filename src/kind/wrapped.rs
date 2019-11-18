use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

use std::sync::{Arc, Mutex};

impl<T> Kind for Arc<Mutex<T>>
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
                .send(
                    channel
                        .fork::<T>(
                            Arc::try_unwrap(self)
                                .map_err(|_| panic!())
                                .unwrap()
                                .into_inner()
                                .unwrap(),
                        )
                        .await?,
                )
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.unwrap();
            Ok(Arc::new(Mutex::new(
                channel.get_fork(handle).await?,
            )))
        })
    }
}

impl<T> Kind for Box<T>
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
                .send(channel.fork::<T>(*self).await.unwrap())
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.unwrap();
            Ok(Box::new(channel.get_fork(handle).await.unwrap()))
        })
    }
}
