use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use failure::Fail;
use futures::{future::BoxFuture, SinkExt, StreamExt};

use super::WrappedError;

#[derive(Fail, Debug)]
pub enum ResultError<T: Fail, E: Fail> {
    #[fail(display = "{}", _0)]
    Ok(T),
    #[fail(display = "{}", _0)]
    Err(E),
}

impl<T, E> Kind for Result<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = Result<ForkHandle, ForkHandle>;
    type ConstructError = WrappedError<ResultError<T::ConstructError, E::ConstructError>>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<ResultError<T::DeconstructError, E::DeconstructError>>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(match self {
                    Ok(item) => Ok(channel.fork(item).await.map_err(ResultError::Ok)?),
                    Err(item) => Err(channel.fork(item).await.map_err(ResultError::Err)?),
                })
                .await
                .map_err(From::from)
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
                    Ok(item) => Ok(channel.get_fork(item).await.map_err(ResultError::Ok)?),
                    Err(item) => Err(channel.get_fork(item).await.map_err(ResultError::Err)?),
                },
            )
        })
    }
}
