use super::{Director, DirectorError};
use crate::{Channel, Channels, Protocol};
use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{self, Poll},
};
use futures::{future::MapErr, Sink, Stream, TryFutureExt};

pub struct Context;

pub struct Empty(Context);

impl Sink<Infallible> for Empty {
    type Error = Infallible;

    fn poll_ready(self: Pin<&mut Self>, _: &mut task::Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: core::pin::Pin<&mut Self>, _: Infallible) -> Result<(), Self::Error> {
        panic!("received empty type `core::convert::Infallible`")
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut task::Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut task::Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl Stream for Empty {
    type Item = Infallible;

    fn poll_next(self: Pin<&mut Self>, _: &mut task::Context) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
    }
}

impl Deref for Empty {
    type Target = Context;

    fn deref(&self) -> &Context {
        &self.0
    }
}

impl DerefMut for Empty {
    fn deref_mut(&mut self) -> &mut Context {
        &mut self.0
    }
}

impl Channel<Infallible, Infallible, Context> for Empty {}

impl Channels<Infallible, Infallible> for Context {
    type Unravel = Empty;
    type Coalesce = Empty;
}

pub struct Null;

impl<P: Protocol<Context, Unravel = Infallible, Coalesce = Infallible>, T> Director<P, T> for Null {
    type Context = Context;
    type UnravelError = Infallible;
    type Unravel =
        MapErr<P::UnravelFuture, fn(P::UnravelError) -> DirectorError<Infallible, P::UnravelError>>;
    type CoalesceError = Infallible;
    type Coalesce = MapErr<
        P::CoalesceFuture,
        fn(P::CoalesceError) -> DirectorError<Infallible, P::CoalesceError>,
    >;

    fn unravel(self, protocol: P, _: T) -> Self::Unravel {
        use DirectorError::Protocol;
        protocol.unravel(Empty(Context)).map_err(Protocol)
    }

    fn coalesce(self, _: T) -> Self::Coalesce {
        use DirectorError::Protocol;
        P::coalesce(Empty(Context)).map_err(Protocol)
    }
}
