use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::Future,
    ConstructResult, DeconstructResult, Kind,
};

use failure::Fail;
use futures::{SinkExt, StreamExt};

use super::WrappedError;

#[derive(Fail, Debug)]
pub enum ResultError<T: Fail, E: Fail> {
    #[fail(display = "{}", _0)]
    Ok(#[fail(cause)] T),
    #[fail(display = "{}", _0)]
    Err(#[fail(cause)] E),
}

#[kind]
impl<T, E> Kind for Result<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = Result<ForkHandle, ForkHandle>;
    type ConstructError = WrappedError<ResultError<T::ConstructError, E::ConstructError>>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<ResultError<T::DeconstructError, E::DeconstructError>>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
                .send(match self {
                    Ok(item) => Ok(channel.fork(item).await.map_err(ResultError::Ok)?),
                    Err(item) => Err(channel.fork(item).await.map_err(ResultError::Err)?),
                })
                .await
                .map_err(WrappedError::Send)?)
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
