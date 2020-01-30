use crate::{Channels, Context as PContext, ContextError, Join, Pass, Protocol, Spawn};
use core::{
    future::Future,
    mem::replace,
    pin::Pin,
    task::{Context, Poll},
};
use futures::{
    future::{ready, Either, Ready},
    ready,
    stream::{once, Forward, Next, Once},
    StreamExt, TryFuture,
};
use pin_utils::pin_mut;
use void::Void;

pub enum Error<Unravel, Send> {
    Unravel(Unravel),
    Send(Send),
}

pub enum Coalesce<'a, C: Channels<<C as PContext>::Handle, Void> + Pass<'a, T>, T: Protocol<'a, C>>
{
    Next(C::Coalesce, Next<'a, C::Coalesce>),
    Join(<C as Join<'a, T>>::Output),
}

pub enum Unravel<'a, C: Pass<'a, T> + Channels<<C as PContext>::Handle, Void>, T: Protocol<'a, C>> {
    Spawn(C::Unravel, <C as Spawn<'a, T>>::Output),
    Send(Forward<Once<Ready<Result<C::Handle, C::SinkError>>>, C::Unravel>),
}

impl<'a, C: Pass<'a, T> + Channels<<C as PContext>::Handle, Void>, T: Protocol<'a, C>>
    Coalesce<'a, C, T>
where
    C::Coalesce: Unpin + Clone,
{
    fn new(channel: &'a mut C::Coalesce) -> Self {
        Coalesce::Next(channel.clone(), channel.next())
    }
}

impl<'a, C: Pass<'a, T> + Channels<<C as PContext>::Handle, Void>, T: Protocol<'a, C>>
    Unravel<'a, C, T>
where
    C::Unravel: Clone,
{
    fn new(channel: &'a mut C::Unravel, item: T) -> Self {
        Unravel::Spawn(channel.clone(), channel.spawn(item))
    }
}

impl<'a, C: Channels<<C as PContext>::Handle, Void> + Pass<'a, T>, T: Unpin + Protocol<'a, C>>
    Future for Coalesce<'a, C, T>
where
    <C as PContext>::Handle: Unpin,
    <C as Join<'a, T>>::Output: Unpin,
    C::Coalesce: Unpin,
{
    type Output = Result<
        Option<T>,
        ContextError<
            <C as Join<'a, T>>::Error,
            <<T as Protocol<'a, C>>::CoalesceFuture as TryFuture>::Error,
        >,
    >;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                Coalesce::Next(channel, item) => {
                    pin_mut!(item);
                    let handle = ready!(item.poll(ctx));
                    let handle = match handle {
                        Some(handle) => handle,
                        None => return Poll::Ready(Ok(None)),
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

impl<'a, C: Channels<<C as PContext>::Handle, Void> + Pass<'a, T>, T: Unpin + Protocol<'a, C>>
    Future for Unravel<'a, C, T>
where
    <C as PContext>::Handle: Unpin,
    <C as Spawn<'a, T>>::Output: Unpin,
    C::Unravel: Clone + Unpin,
{
    type Output = Result<
        (),
        Error<
            ContextError<
                <C as Spawn<'a, T>>::Error,
                <<T as Protocol<'a, C>>::UnravelFuture as TryFuture>::Error,
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
                        Unravel::Send(once(ready(Ok(handle))).forward(channel.clone()));
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

impl<'a, C: Channels<<C as PContext>::Handle, Void> + Pass<'a, T>, T: Unpin + Protocol<'a, C>>
    Protocol<'a, C> for Option<T>
where
    C::Handle: Unpin,
    <C as Join<'a, T>>::Output: Unpin,
    <C as Spawn<'a, T>>::Output: Unpin,
    <C as Channels<<C as PContext>::Handle, Void>>::Coalesce: Clone + Unpin + 'a,
    <C as Channels<<C as PContext>::Handle, Void>>::Unravel: Clone + Unpin,
{
    type Unravel = C::Handle;
    type UnravelFuture =
        Either<Unravel<'a, C, T>, Ready<Result<(), <Unravel<'a, C, T> as TryFuture>::Error>>>;
    type Coalesce = Void;
    type CoalesceFuture = Coalesce<'a, C, T>;

    fn unravel(
        self,
        channel: &'a mut <C as Channels<<C as PContext>::Handle, Void>>::Unravel,
    ) -> Self::UnravelFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static,
    {
        if let Some(item) = self {
            Either::Left(Unravel::new(channel, item))
        } else {
            Either::Right(ready(Ok(())))
        }
    }

    fn coalesce(
        channel: &'a mut <C as Channels<<C as PContext>::Handle, Void>>::Coalesce,
    ) -> Self::CoalesceFuture
    where
        C: Channels<Self::Unravel, Self::Coalesce> + 'static,
    {
        Coalesce::new(channel)
    }
}
