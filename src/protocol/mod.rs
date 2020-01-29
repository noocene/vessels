use futures::{Sink, Stream, TryFuture};
use std::{future::Future, ops::DerefMut};

mod option;

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

pub trait Channel<T, U, E>: Stream<Item = T> + Sink<U, Error = E> + Clone + Send + Unpin {}

pub trait Context: Sized {
    type Handle;
    type SinkError;
}

pub trait Transport<Unravel, Coalesce>: Context + Sized {
    type Unravel: Channel<Coalesce, Unravel, Self::SinkError> + DerefMut<Target = Self>;
    type Coalesce: Channel<Unravel, Coalesce, Self::SinkError> + DerefMut<Target = Self>;
}

pub trait Protocol<C: Context>: Sized {
    type Unravel;
    type UnravelFuture: TryFuture<Ok = ()>;
    type Coalesce;
    type CoalesceFuture: TryFuture<Ok = Self>;

    fn unravel(self, channel: &mut C::Unravel) -> Self::UnravelFuture
    where
        C: Transport<Self::Unravel, Self::Coalesce> + 'static;

    fn coalesce(channel: &mut C::Coalesce) -> Self::CoalesceFuture
    where
        C: Transport<Self::Unravel, Self::Coalesce> + 'static;
}
