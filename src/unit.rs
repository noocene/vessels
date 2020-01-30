use crate::{Channels, Context, Protocol};
use futures::future::{ready, Ready};
use void::Void;

impl<C: Context> Protocol<C> for () {
    type Unravel = Void;
    type UnravelFuture = Ready<Result<(), Void>>;
    type Coalesce = Void;
    type CoalesceFuture = Ready<Result<(), Void>>;

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
        ready(Ok(()))
    }
}
