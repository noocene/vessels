use super::WrappedError;
use crate::{
    channel::{Channel, ForkHandle},
    ConstructResult, DeconstructResult, Kind,
};

use failure::{Error, Fail};
use futures::{future::BoxFuture, SinkExt, StreamExt};
use std::fmt::{self, Debug, Display, Formatter};
use void::Void;

#[derive(Kind)]
struct ErrorShim {
    name: Option<String>,
    cause: Option<Box<ErrorShim>>,
    debug: String,
    display: String,
}

impl Display for ErrorShim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl Debug for ErrorShim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl Fail for ErrorShim {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|item| item.as_str())
    }
    fn cause(&self) -> Option<&dyn Fail> {
        self.cause.as_ref().map(|item| item.as_ref() as &dyn Fail)
    }
}

impl ErrorShim {
    fn from_fail<F: Fail + ?Sized>(failure: &F) -> Self {
        ErrorShim {
            name: failure.name().map(str::to_owned),
            cause: failure.cause().map(|e| Box::new(ErrorShim::from_fail(e))),
            debug: format!("{:?}", failure),
            display: format!("{}", failure),
        }
    }
}

impl Kind for Error {
    type ConstructItem = ForkHandle;
    type ConstructError = WrappedError<Void>;
    type ConstructFuture = BoxFuture<'static, ConstructResult<Self>>;
    type DeconstructItem = ();
    type DeconstructError = WrappedError<Void>;
    type DeconstructFuture = BoxFuture<'static, DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            channel
                .send(channel.fork(ErrorShim::from_fail(self.as_fail())).await?)
                .await
                .map_err(From::from)
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
            Ok(channel.get_fork::<ErrorShim>(handle).await?.into())
        })
    }
}
