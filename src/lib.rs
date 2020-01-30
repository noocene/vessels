#![no_std]

use core::{future::Future, ops::DerefMut};
use futures::{Sink, Stream, TryFuture};

mod array;
mod option;
mod unit;

pub enum ContextError<Context, Protocol> {
    Context(Context),
    Protocol(Protocol),
}

pub trait Join<'a, P: Protocol<'a, Self>>: Context {
    type Error;
    type Output: Future<
        Output = Result<P, ContextError<Self::Error, <P::CoalesceFuture as TryFuture>::Error>>,
    >;

    fn join(&mut self, handle: Self::Handle) -> Self::Output;
}

pub trait Spawn<'a, P: Protocol<'a, Self>>: Context {
    type Error;
    type Output: Future<
        Output = Result<
            Self::Handle,
            ContextError<Self::Error, <P::UnravelFuture as TryFuture>::Error>,
        >,
    >;

    fn spawn(&mut self, item: P) -> Self::Output;
}

pub trait Pass<'a, P: Protocol<'a, Self>>: Spawn<'a, P> + Join<'a, P> {}

impl<'a, P: Protocol<'a, T>, T: Spawn<'a, P> + Join<'a, P>> Pass<'a, P> for T {}

pub trait Channel<T, U, S: Context>:
    Stream<Item = T> + Sink<U, Error = S::SinkError> + DerefMut<Target = S>
{
}

pub trait Context: Sized {
    type Handle;
    type SinkError;
}

pub trait Channels<Unravel, Coalesce>: Context + Sized {
    type Unravel: Channel<Coalesce, Unravel, Self>;
    type Coalesce: Channel<Unravel, Coalesce, Self>;
}

pub trait Protocol<'a, C: Context>: Sized {
    type Unravel;
    type UnravelFuture: TryFuture<Ok = ()>;
    type Coalesce;
    type CoalesceFuture: TryFuture<Ok = Self>;

    fn unravel(self, channel: &'a mut C::Unravel) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static;

    fn coalesce(channel: &'a mut C::Coalesce) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static;
}
