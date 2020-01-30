#![no_std]

use core::{future::Future, ops::DerefMut};
use futures::{Sink, Stream, TryFuture};

mod array;
pub mod director;
pub use director::Director;
mod option;
mod unit;

#[derive(Debug)]
pub enum ContextError<Context, Protocol> {
    Context(Context),
    Protocol(Protocol),
}

pub trait Join<P: Protocol<Self>>: Dispatch {
    type Error;
    type Output: Future<
        Output = Result<P, ContextError<Self::Error, <P::CoalesceFuture as TryFuture>::Error>>,
    >;

    fn join(&mut self, handle: Self::Handle) -> Self::Output;
}

pub trait Spawn<P: Protocol<Self>>: Dispatch {
    type Error;
    type Output: Future<
        Output = Result<
            Self::Handle,
            ContextError<Self::Error, <P::UnravelFuture as TryFuture>::Error>,
        >,
    >;

    fn spawn(&mut self, item: P) -> Self::Output;
}

pub trait Pass<P: Protocol<Self>>: Spawn<P> + Join<P> {}

impl<P: Protocol<T>, T: Spawn<P> + Join<P>> Pass<P> for T {}

pub trait Channel<T, U, S: ?Sized>: Stream<Item = T> + Sink<U> + DerefMut<Target = S> {}

pub trait Dispatch {
    type Handle;
}

pub trait Channels<Unravel, Coalesce> {
    type Unravel: Channel<Coalesce, Unravel, Self>;
    type Coalesce: Channel<Unravel, Coalesce, Self>;
}

pub trait Protocol<C: ?Sized>: Sized {
    type Unravel;
    type UnravelError;
    type UnravelFuture: Future<Output = Result<(), Self::UnravelError>>;
    type Coalesce;
    type CoalesceError;
    type CoalesceFuture: Future<Output = Result<Self, Self::CoalesceError>>;

    fn unravel(self, channel: C::Unravel) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>;

    fn coalesce(channel: C::Coalesce) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>;
}
