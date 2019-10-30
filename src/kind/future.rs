use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use serde::{Deserialize, Serialize};

use futures::{
    future::{ready, BoxFuture},
    task::Context,
    Future as IFuture, Poll, SinkExt, StreamExt,
};

use std::pin::Pin;

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug)]
pub enum KResult {
    Ok(ForkHandle),
    Err(ForkHandle),
}

pub struct Future<T: Kind>(BoxFuture<'static, T>);

impl<T: Kind> IFuture for Future<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}

impl<T: Kind> Future<T> {
    pub fn new<F: IFuture<Output = T> + Send + 'static>(future: F) -> Self {
        Future(Box::pin(future))
    }
}

impl<T> Kind for Future<T>
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
                .send(channel.fork(self.await).await.unwrap())
                .await
                .map_err(|_| panic!())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            let handle = channel.next().await.unwrap();
            Ok(Future::new(ready(
                channel.get_fork::<T>(handle).await.unwrap(),
            )))
        })
    }
}
