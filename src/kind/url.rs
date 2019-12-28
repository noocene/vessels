use crate::{channel::Channel, kind, kind::Future, ConstructResult, DeconstructResult, Kind};

use futures::{SinkExt, StreamExt};
use url::{ParseError, Url};

use super::WrappedError;

use void::Void;

#[kind]
impl Kind for Url {
    type ConstructItem = String;
    type ConstructError = WrappedError<ParseError>;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<Void>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            Ok(channel
                .send(self.into_string())
                .await
                .map_err(WrappedError::Send)?)
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        mut channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(channel
                .next()
                .await
                .ok_or(WrappedError::Insufficient {
                    got: 0,
                    expected: 1,
                })?
                .parse()?)
        })
    }
}
