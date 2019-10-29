use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, Kind,
};

use serde::{Deserialize, Serialize};

use futures::{
    future::{ok, BoxFuture, ready},
    stream::once,
    task::Context,
    Future as IFuture, FutureExt, Poll, SinkExt, StreamExt, TryFutureExt,
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
    type Error = T::Error;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructFuture = BoxFuture<'static, ()>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(self.then(move |v| {
            channel.fork(v).then(|handle| {
                let channel = channel.sink_map_err(|_| panic!());
                Box::pin(
                    once(ok(handle))
                        .forward(channel)
                        .unwrap_or_else(|_| panic!()),
                )
            })
        }))
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(
            channel
                .into_future()
                .then(move |(item, channel)| {
                       channel
                            .get_fork::<T>(item.unwrap())
                            .map_ok(|t| Future::new(ready(t)))
                })
        )
    }
}
