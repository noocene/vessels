use crate::{Channels, Context as PContext, ContextError, Join, Pass, Protocol, Spawn};
use core::convert::Infallible;
use core::{
    future::Future,
    mem::replace,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    future::{ready, Either, Ready},
    ready,
    stream::{once, Forward, Once, StreamFuture},
    StreamExt, TryFuture,
};
use pin_utils::pin_mut;

pub enum Error<Unravel, Send> {
    Unravel(Unravel),
    Send(Send),
}

pub enum Coalesce<C: Channels<<C as PContext>::Handle, Infallible> + Pass<T>, T: Protocol<C>> {
    Next(StreamFuture<C::Coalesce>),
    Join(<C as Join<T>>::Output),
}

pub enum Unravel<C: Pass<T> + Channels<<C as PContext>::Handle, Infallible>, T: Protocol<C>> {
    Spawn(Option<C::Unravel>, <C as Spawn<T>>::Output),
    Send(Forward<Once<Ready<Result<C::Handle, C::SinkError>>>, C::Unravel>),
}

impl<C: Pass<T> + Channels<<C as PContext>::Handle, Infallible>, T: Protocol<C>> Coalesce<C, T>
where
    C::Coalesce: Unpin,
{
    fn new(channel: C::Coalesce) -> Self {
        Coalesce::Next(channel.into_future())
    }
}

impl<C: Pass<T> + Channels<<C as PContext>::Handle, Infallible>, T: Protocol<C>> Unravel<C, T> {
    fn new(mut channel: C::Unravel, item: T) -> Self {
        let spawn = channel.spawn(item);
        Unravel::Spawn(Some(channel), spawn)
    }
}

impl<C: Channels<<C as PContext>::Handle, Infallible> + Pass<T>, T: Unpin + Protocol<C>> Future
    for Coalesce<C, T>
where
    <C as PContext>::Handle: Unpin,
    <C as Join<T>>::Output: Unpin,
    C::Coalesce: Unpin,
{
    type Output = Result<
        Option<T>,
        ContextError<
            <C as Join<T>>::Error,
            <<T as Protocol<C>>::CoalesceFuture as TryFuture>::Error,
        >,
    >;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Coalesce::Next(next) => {
                    pin_mut!(next);
                    let handle = ready!(next.poll(ctx));
                    let (handle, mut channel) = match handle {
                        (Some(handle), channel) => (handle, channel),
                        (None, _) => return Poll::Ready(Ok(None)),
                    };
                    let replacement = Coalesce::Join(channel.join(handle));
                    replace(&mut *self, replacement);
                }
                Coalesce::Join(join) => {
                    pin_mut!(join);
                    return Poll::Ready(ready!(join.poll(ctx)).map(Some));
                }
            };
        }
    }
}

impl<C: Channels<<C as PContext>::Handle, Infallible> + Pass<T>, T: Unpin + Protocol<C>> Future
    for Unravel<C, T>
where
    <C as PContext>::Handle: Unpin,
    <C as Spawn<T>>::Output: Unpin,
    C::Unravel: Unpin,
{
    type Output = Result<
        (),
        Error<
            ContextError<
                <C as Spawn<T>>::Error,
                <<T as Protocol<C>>::UnravelFuture as TryFuture>::Error,
            >,
            C::SinkError,
        >,
    >;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Unravel::Spawn(channel, item) => {
                    pin_mut!(item);
                    let handle = ready!(item.poll(ctx));
                    let handle = match handle {
                        Ok(handle) => handle,
                        Err(e) => return Poll::Ready(Err(Error::Unravel(e))),
                    };
                    let replacement =
                        Unravel::Send(once(ready(Ok(handle))).forward(channel.take().expect(
                            "violated invariant in Protocol for Option: no channel in Spawn stage",
                        )));
                    replace(&mut *self, replacement);
                }
                Unravel::Send(send) => {
                    pin_mut!(send);
                    return Poll::Ready(ready!(send.poll(ctx)).map_err(Error::Send));
                }
            };
        }
    }
}

impl<C: Channels<<C as PContext>::Handle, Infallible> + Pass<T>, T: Unpin + Protocol<C>> Protocol<C>
    for Option<T>
where
    C::Handle: Unpin,
    <C as Join<T>>::Output: Unpin,
    <C as Spawn<T>>::Output: Unpin,
    <C as Channels<<C as PContext>::Handle, Infallible>>::Coalesce: Unpin,
    <C as Channels<<C as PContext>::Handle, Infallible>>::Unravel: Unpin,
{
    type Unravel = C::Handle;
    type UnravelError = <Unravel<C, T> as TryFuture>::Error;
    type UnravelFuture = Either<Unravel<C, T>, Ready<Result<(), Self::UnravelError>>>;
    type Coalesce = Infallible;
    type CoalesceError = <Coalesce<C, T> as TryFuture>::Error;
    type CoalesceFuture = Coalesce<C, T>;

    fn unravel(
        self,
        channel: <C as Channels<<C as PContext>::Handle, Infallible>>::Unravel,
    ) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        if let Some(item) = self {
            Either::Left(Unravel::new(channel, item))
        } else {
            Either::Right(ready(Ok(())))
        }
    }

    fn coalesce(
        channel: <C as Channels<<C as PContext>::Handle, Infallible>>::Coalesce,
    ) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce>,
    {
        Coalesce::new(channel)
    }
}
