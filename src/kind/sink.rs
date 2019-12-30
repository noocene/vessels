use crate::{
    channel::{Channel, ForkHandle},
    kind,
    kind::{Future, Sink},
    ConstructResult, DeconstructResult, Kind,
};

use futures::{
    future::ready,
    lock::Mutex,
    task::{Context, Poll},
    Sink as ISink, SinkExt, StreamExt,
};

use super::WrappedError;

use alloc::sync::Arc;
use core::{marker::PhantomData, pin::Pin};

use void::Void;

pub struct KindSink<T: Kind, E: Kind, C: Channel<ForkHandle, ForkHandle>> {
    channel: Arc<Mutex<C>>,
    _marker: PhantomData<(T, E)>,
    item: Future<()>,
}

impl<T: Kind, E: Kind, C: Channel<ForkHandle, ForkHandle>> ISink<T> for KindSink<T, E, C> {
    type Error = E;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let poll = self.item.as_mut().poll(cx).map(Ok);
        if let Poll::Ready(_) = poll {
            self.item = Box::pin(ready(()));
        }
        poll
    }

    fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let channel = self.channel.clone();
        self.item = Box::pin(async move {
            let mut channel = channel.lock().await;
            let handle = channel.fork(item).await.unwrap();
            channel.send(handle).await.unwrap_or_else(|_| panic!());
        });
        Ok(())
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let poll = self.item.as_mut().poll(cx).map(Ok);
        if let Poll::Ready(_) = poll {
            self.item = Box::pin(ready(()));
        }
        poll
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let poll = self.item.as_mut().poll(cx).map(Ok);
        if let Poll::Ready(_) = poll {
            self.item = Box::pin(ready(()));
        }
        poll
    }
}

#[kind]
impl<T, E> Kind for Sink<T, E>
where
    T: Kind,
    E: Kind,
{
    type ConstructItem = ForkHandle;
    type ConstructError = Void;
    type ConstructFuture = Future<ConstructResult<Self>>;
    type DeconstructItem = ForkHandle;
    type DeconstructError = WrappedError<Void>;
    type DeconstructFuture = Future<DeconstructResult<Self>>;
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        mut self,
        mut channel: C,
    ) -> Self::DeconstructFuture {
        Box::pin(async move {
            while let Some(handle) = channel.next().await {
                if let Err(error) = self
                    .send(channel.get_fork::<T>(handle).await.unwrap())
                    .await
                {
                    let handle = channel.fork::<E>(error).await.unwrap();
                    channel.send(handle).await.map_err(WrappedError::Send)?;
                }
            }
            Ok(())
        })
    }
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture {
        Box::pin(async move {
            Ok(Box::pin(KindSink {
                channel: Arc::new(Mutex::new(channel)),
                _marker: PhantomData,
                item: Box::pin(ready(())),
            }) as Sink<T, E>)
        })
    }
}
