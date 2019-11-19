use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

use std::sync::{Arc, Mutex};

use super::{ConstructError, DeconstructError};

impl<T> Kind for Arc<Mutex<T>>
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = ConstructError<T::ConstructError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = DeconstructError<T::DeconstructError>;
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
                .map_err(From::from)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(ConstructError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok(Arc::new(Mutex::new(channel.get_fork(handle).await?)))
        })
    }
}

impl<T> Kind for Box<T>
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = ConstructError<T::ConstructError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = DeconstructError<T::DeconstructError>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(channel.fork::<T>(*self).await?)
                .await
                .map_err(From::from)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(ConstructError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok(Box::new(channel.get_fork(handle).await?))
        })
    }
}
