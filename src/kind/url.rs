use crate::{channel::Channel, ConstructResult, DeconstructResult, Kind};

use futures::{future::BoxFuture, SinkExt, StreamExt};
use url::{ParseError, Url};

use super::WrappedError;

use void::Void;

impl Kind for Url {
    type ConstructItem = String;
    type ConstructError = WrappedError<ParseError>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<Void>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move { Ok(channel.send(self.into_string()).await?) })
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
