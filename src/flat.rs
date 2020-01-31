use crate::{Channels, Protocol};
use core::convert::Infallible;
use futures::{
    future::{ready, Ready},
    Sink,
};

pub trait Flat {}

pub struct InsufficientData;

impl<C: Channels<T, Infallible>, T: Flat> Protocol<C> for T {
    type Unravel = T;
    type UnravelError = <C::Unravel as Sink<T>>::Error;
    type UnravelFuture = Ready<Result<(), Self::UnravelError>>;
    type Coalesce = Infallible;
    type CoalesceError = InsufficientData;
    type CoalesceFuture = Ready<Result<T, InsufficientData>>;

    fn unravel(self, _: C::Unravel) -> Self::UnravelFuture {
        ready(Ok(()))
    }

    fn coalesce(_: C::Coalesce) -> Self::CoalesceFuture {
        ready(Ok(()))
    }
}
