use futures::{Async, AsyncSink, Poll, StartSend};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

pub use vitruvia_derive::protocol;

pub struct Context<T: Value> {
    item: PhantomData<T>,
}

impl<T: Value> Context<T> {
    pub fn new() -> Self {
        Context { item: PhantomData }
    }
}

impl<T: Value> futures::Sink for Context<T> {
    type SinkError = ();
    type SinkItem = T;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::NotReady)
    }
}

impl<T: Value> futures::Stream for Context<T> {
    type Error = ();
    type Item = T;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::NotReady)
    }
}

pub trait Value {
    type Item: Serialize + DeserializeOwned;

    fn construct(context: Context<Self::Item>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}

pub struct Future<T: Value, E: Value> {
    item: PhantomData<T>,
    error: PhantomData<E>,
}

impl<T: Value, E: Value> Value for Future<T, E> {
    type Item = ();
}

impl<T: Value, E: Value> futures::Future for Future<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::NotReady)
    }
}

pub struct Object<T: ?Sized> {
    inner: Box<T>,
}

impl<T: ?Sized> Value for Object<T> {
    type Item = ();
}

impl<T> Value for T
where
    T: Serialize + DeserializeOwned,
{
    type Item = ();
}
