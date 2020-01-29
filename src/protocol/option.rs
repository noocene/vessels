use crate::protocol::{ContextError, Join, Pass, Protocol, Spawn, Transport};
use futures::{future::BoxFuture, SinkExt, StreamExt, TryFuture};
use void::Void;

pub enum Error<Unravel, Send> {
    Unravel(Unravel),
    Send(Send),
}

impl<C: Send + Pass<T>, T: Protocol<C> + Send + 'static> Protocol<C> for Option<T>
where
    C::Handle: Send,
    <C as Spawn<T>>::Output: Send,
    <C as Spawn<T>>::Error: Send,
    <C as Join<T>>::Output: Send,
    <C as Join<T>>::Error: Send,
    C::SinkError: Send,
    <<T as Protocol<C>>::UnravelFuture as TryFuture>::Error: Send,
    <<T as Protocol<C>>::CoalesceFuture as TryFuture>::Error: Send,
{
    type Unravel = C::Handle;
    type UnravelFuture = BoxFuture<
        'static,
        Result<
            (),
            Error<
                ContextError<
                    <C as Spawn<T>>::Error,
                    <<T as Protocol<C>>::UnravelFuture as TryFuture>::Error,
                >,
                C::SinkError,
            >,
        >,
    >;
    type Coalesce = Void;
    type CoalesceFuture = BoxFuture<
        'static,
        Result<
            Option<T>,
            ContextError<
                <C as Join<T>>::Error,
                <<T as Protocol<C>>::CoalesceFuture as TryFuture>::Error,
            >,
        >,
    >;

    fn unravel(self, channel: &mut C::Unravel) -> Self::UnravelFuture
    where
        C: Transport<Self::Unravel, Self::Coalesce> + 'static,
    {
        let mut channel = channel.clone();

        Box::pin(async move {
            use Error::{Send, Unravel};

            if let Some(item) = self {
                let handle = channel.spawn(item).await.map_err(Unravel)?;
                channel.send(handle).await.map_err(Send)?
            }

            Ok(())
        })
    }

    fn coalesce(channel: &mut C::Coalesce) -> Self::CoalesceFuture
    where
        C: Transport<Self::Unravel, Self::Coalesce> + 'static,
    {
        let mut channel = channel.clone();

        Box::pin(async move {
            if let Some(handle) = channel.next().await {
                Ok(Some(channel.join(handle).await?))
            } else {
                Ok(None)
            }
        })
    }
}
