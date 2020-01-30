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

pub trait Join<P: Protocol<Self>>: Context {
    type Error;
    type Output: Future<
        Output = Result<P, ContextError<Self::Error, <P::CoalesceFuture as TryFuture>::Error>>,
    >;

    fn join(&mut self, handle: Self::Handle) -> Self::Output;
}

pub trait Spawn<P: Protocol<Self>>: Context {
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

pub trait Channel<T, U, S: ?Sized + Context>:
    Stream<Item = T> + Sink<U, Error = S::SinkError> + DerefMut<Target = S>
{
}

pub trait Context {
    type Handle;
    type SinkError;
}

pub trait Channels<Unravel, Coalesce>: Context {
    type Unravel: Channel<Coalesce, Unravel, Self>;
    type Coalesce: Channel<Unravel, Coalesce, Self>;
}

pub trait Protocol<C: ?Sized>: Sized {
    type Unravel;
    type UnravelFuture: TryFuture<Ok = ()>;
    type Coalesce;
    type CoalesceFuture: TryFuture<Ok = Self>;

    fn unravel(self, channel: C::Unravel) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static;

    fn coalesce(channel: C::Coalesce) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static;
}

pub enum DirectorError<T, U> {
    Director(T),
    Protocol(U),
}

trait Director<P: Protocol<Self::Context>, Transport> {
    type Context: Channels<P::Unravel, P::Coalesce>;
    type UnravelError;
    type Unravel: TryFuture<
        Ok = (),
        Error = DirectorError<Self::UnravelError, <P::UnravelFuture as TryFuture>::Error>,
    >;
    type CoalesceError;
    type Coalesce: TryFuture<
        Ok = P,
        Error = DirectorError<Self::CoalesceError, <P::CoalesceFuture as TryFuture>::Error>,
    >;

    fn unravel(protocol: P, transport: Transport) -> Self::Unravel;
    fn coalesce(transport: Transport) -> Self::Coalesce;
}
