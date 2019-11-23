use serde::{de::DeserializeOwned, Serialize};

use futures::{SinkExt, StreamExt};

use crate::{channel::Channel, kind::Future, ConstructResult, DeconstructResult, Kind};

use super::{using, AsKind, WrappedError};

use std::ops::Deref;

use void::Void;

#[derive(Clone, Debug, Copy, Hash, Eq, Ord, PartialOrd, PartialEq, Default)]
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

impl<T: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static> AsKind<using::Serde> for T {
    type Kind = Serde<T>;

    fn into_kind(self) -> Serde<T> {
        Serde(self)
    }
    fn from_kind(kind: Self::Kind) -> Self {
        kind.0
    }
}

impl<T: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static> Kind for Serde<T> {
    type ConstructItem = T;
    type ConstructError = WrappedError<Void>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<Void>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;

    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move { Ok(channel.send(self.0).await?) })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(Serde(channel.next().await.ok_or(
                WrappedError::Insufficient {
                    got: 0,
                    expected: 1,
                },
            )?))
        })
    }
}
