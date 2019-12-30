use crate::{
    kind::{Sink, Stream},
    Kind,
};
use core::pin::Pin;
use futures::{
    task::{Context, Poll},
    Sink as ISink, Stream as IStream,
};

#[derive(Kind)]
pub struct SinkStream<T, E, U>(Sink<T, E>, Stream<U>);

impl<T, E, U> SinkStream<T, E, U> {
    pub fn new<
        R: ISink<T, Error = E> + Sync + Send + 'static,
        S: IStream<Item = U> + Sync + Send + 'static,
    >(
        sink: R,
        stream: S,
    ) -> Self {
        SinkStream(Box::pin(sink), Box::pin(stream))
    }
}

impl<T, E, U> ISink<T> for SinkStream<T, E, U> {
    type Error = E;

    fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.0.as_mut().start_send(item)
    }
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.0.as_mut().poll_ready(cx)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.0.as_mut().poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.0.as_mut().poll_close(cx)
    }
}

impl<T, E, U> IStream for SinkStream<T, E, U> {
    type Item = U;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.1.as_mut().poll_next(cx)
    }
}
