use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{future::BoxFuture, SinkExt, StreamExt};

use std::sync::{Arc, Mutex};

use super::WrappedError;

impl<T> Kind for Arc<Mutex<T>>
where
    T: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = WrappedError<T::ConstructError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<T::DeconstructError>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
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
                .await?)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(WrappedError::Insufficient {
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
    type ConstructError = WrappedError<T::ConstructError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<T::DeconstructError>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move { Ok(channel.send(channel.fork::<T>(*self).await?).await?) })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.ok_or(WrappedError::Insufficient {
                got: 0,
                expected: 1,
            })?;
            Ok(Box::new(channel.get_fork(handle).await?))
        })
    }
}
