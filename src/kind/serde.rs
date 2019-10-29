use serde::{de::DeserializeOwned, Serialize};

use futures::{
    future::{ok, BoxFuture},
    stream::once,
    FutureExt, SinkExt, StreamExt, TryFutureExt,
};

use crate::{channel::Channel, Kind};

use super::{using, AsKind};

use std::ops::Deref;

pub struct Serde<T: Serialize + DeserializeOwned + Send + 'static>(pub T);

impl<T: Serialize + DeserializeOwned + Send + 'static> Serde<T> {
    pub fn new(item: T) -> Self {
        Serde(item)
    }
}

impl<T: Serialize + DeserializeOwned + Send + 'static> Deref for Serde<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize + DeserializeOwned + Send + 'static> From<T> for Serde<T> {
    fn from(item: T) -> Self {
        Serde(item)
    }
}

impl<T: Serialize + DeserializeOwned + Send + Unpin + 'static> AsKind<using::Serde> for T {
    type Kind = Serde<T>;
    type ConstructFuture = BoxFuture<'static, Result<T, <Serde<T> as Kind>::Error>>;

    fn into_kind(self) -> Serde<T> {
        Serde(self)
    }
    fn from_kind(future: <Serde<T> as Kind>::ConstructFuture) -> Self::ConstructFuture {
        Box::pin(future.map_ok(|item| item.0))
    }
}

impl<T: Serialize + DeserializeOwned + Send + Unpin + 'static> Kind for Serde<T> {
    type ConstructItem = T;
    type Error = ();
    type ConstructFuture = BoxFuture<'static, Result<Serde<T>, Self::Error>>;
    type DeconstructItem = ();
    type DeconstructFuture = BoxFuture<'static, ()>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture {
        let channel = channel.sink_map_err(|_| panic!());
        Box::pin(
            once(ok(self.0))
                .forward(channel)
                .unwrap_or_else(|_| panic!()),
        )
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(
            channel
                .into_future()
                .map(|v| Serde(v.0.unwrap()))
                .unit_error(),
        )
    }
}
