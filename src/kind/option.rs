use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::Future,
    ConstructResult, DeconstructResult, Kind,
};

use futures::{SinkExt, StreamExt};

use super::WrappedError;

#[kind]
impl<T> Kind for Option<T>
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
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
                .send(match self {
                    None => None,
                    Some(item) => Some(channel.fork(item).await?),
                })
                .await?)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(
                match channel.next().await.ok_or(WrappedError::Insufficient {
                    got: 0,
                    expected: 1,
                })? {
                    Some(item) => Some(channel.get_fork(item).await?),
                    None => None,
                },
            )
        })
    }
}
