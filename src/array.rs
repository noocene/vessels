use crate::{Channels, Context, Protocol};
use core::convert::Infallible;
use futures::future::{ready, Ready};

impl<C: Context, T> Protocol<C> for [T; 0] {
    type Unravel = Infallible;
    type UnravelError = Infallible;
    type UnravelFuture = Ready<Result<(), Infallible>>;
    type Coalesce = Infallible;
    type CoalesceError = Infallible;
    type CoalesceFuture = Ready<Result<[T; 0], Infallible>>;

    fn unravel(self, _: C::Unravel) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        ready(Ok(()))
    }

    fn coalesce(_: C::Coalesce) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        ready(Ok([]))
    }
}
