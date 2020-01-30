use crate::{Channels, Protocol};
use core::future::Future;
use futures::TryFuture;

mod trivial;
pub use trivial::Trivial;

#[derive(Debug)]
pub enum DirectorError<T, U> {
    Director(T),
    Protocol(U),
}

pub trait Director<P: Protocol<Self::Context>, Unravel, Coalesce> {
    type Context: Channels<P::Unravel, P::Coalesce>;
    type UnravelError;
    type Unravel: Future<
        Output = Result<
            (),
            DirectorError<Self::UnravelError, <P::UnravelFuture as TryFuture>::Error>,
        >,
    >;
    type CoalesceError;
    type Coalesce: Future<
        Output = Result<
            P,
            DirectorError<Self::CoalesceError, <P::CoalesceFuture as TryFuture>::Error>,
        >,
    >;

    fn unravel(self, protocol: P, transport: Unravel) -> Self::Unravel;

    fn coalesce(self, transport: Coalesce) -> Self::Coalesce;
}
