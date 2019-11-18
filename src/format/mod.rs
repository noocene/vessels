#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
#[doc(inline)]
pub use json::Json;
#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "cbor")]
#[doc(inline)]
pub use cbor::Cbor;
#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "bincode")]
#[doc(inline)]
pub use bincode::Bincode;

use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver},
    future::{ok, BoxFuture},
    stream::BoxStream,
    task::{Context as FContext, Poll},
    Future, FutureExt, Sink, SinkExt, Stream, StreamExt, TryFutureExt,
};

use crate::{
    channel::{Context, Shim, Target, Waiter},
    core,
    core::{executor::Spawn, Executor},
    Kind,
};

use serde::{de::DeserializeSeed, Serialize};

use std::{
    fmt::{Debug, Display, Formatter},
    pin::Pin,
};

use failure::Fail;

#[doc(hidden)]
pub struct StreamSink<T, U, E>(
    pub(crate) BoxStream<'static, T>,
    pub(crate) Pin<Box<dyn Sink<U, Error = E> + Send>>,
);

impl<T, U, E> Sink<U> for StreamSink<T, U, E> {
    type Error = E;

    fn start_send(mut self: Pin<&mut Self>, item: U) -> Result<(), Self::Error> {
        self.1.as_mut().start_send(item)
    }
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.1.as_mut().poll_ready(cx)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.1.as_mut().poll_flush(cx)
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Result<(), Self::Error>> {
        self.1.as_mut().poll_close(cx)
    }
}

impl<T, I, U> Stream for StreamSink<T, I, U> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FContext) -> Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

pub trait UniformStreamSink<T>: Sink<T> + Stream<Item = T> {}

impl<T, U> UniformStreamSink<T> for U where U: Sink<T> + Stream<Item = T> {}

/// A serialization format used in the transport of `Kind`s.
///
/// This is generally a minimal wrapper that encapsulates a `serde` format.
pub trait Format {
    /// The underlying representation used by this `Format`, i.e. `Vec<u8>` for most
    /// binary formats and `String` for those of a human-readable nature.
    type Representation;
    /// The failure condition of this format. This may be encountered during deserialization.
    type Error: Fail;

    /// Serializes the provided item.
    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    /// Deserializes an item from the provided formatted representation.
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> BoxFuture<'static, Result<T::Value, (Self::Error, Self::Representation)>>
    where
        T: Send + 'static,
        Self: Sized;
}

pub trait ApplyEncode<'de>:
    Sized + UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output;
}

impl<'de, T> ApplyEncode<'de> for T
where
    T: UniformStreamSink<<Self as Context<'de>>::Item> + Context<'de>,
{
    fn encode<F: Format + Encode<'de, Self>>(self) -> <F as Encode<'de, Self>>::Output {
        <F as Encode<_>>::encode(self)
    }
}

pub trait ApplyDecode<'de, K: Kind> {
    fn decode<T: Target<'de, K> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Clone + Send + 'static,
        <Self as Sink<F::Representation>>::Error: Fail,
        T::Item: Send + 'static;
}

impl<'de, U, K: Kind> ApplyDecode<'de, K> for U {
    fn decode<T: Target<'de, K> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Clone + Send,
        <Self as Sink<F::Representation>>::Error: Fail,
        T::Item: Send,
    {
        <F as Decode<'de, Self, K>>::decode::<T>(self)
    }
}

pub trait Decode<'de, C: UniformStreamSink<<Self as Format>::Representation> + 'static, K: Kind>:
    Format
{
    type Output: Future<Output = Result<K, K::ConstructError>>;

    fn decode<T: Target<'de, K> + Send + 'static>(input: C) -> Self::Output
    where
        T::Item: Send;
}

pub trait Encode<'de, C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de>>:
    Format + Sized
{
    type Output: Stream<Item = <Self as Format>::Representation>
        + Sink<Self::Representation, Error = EncodeError<Self, <C as Context<'de>>::Item, C>>;

    fn encode(input: C) -> Self::Output;
}

impl<
        'de,
        C: Send + UniformStreamSink<<Self as Format>::Representation> + 'static,
        T: Format + 'static,
        K: Kind,
    > Decode<'de, C, K> for T
where
    Self::Representation: Send + Clone,
    <C as Sink<<Self as Format>::Representation>>::Error: Fail,
{
    type Output = BoxFuture<'static, Result<K, K::ConstructError>>;

    fn decode<U: Target<'de, K> + Send + 'static>(input: C) -> Self::Output
    where
        U::Item: Send,
    {
        let shim = U::new_shim();
        let context = shim.context();
        let (sink, stream) = input.split();
        Box::pin(
            shim.complete(StreamSink(
                Box::pin(
                    stream
                        .map(move |item| {
                            let ct = context.clone();
                            Self::deserialize(item, context.clone())
                                .or_else(move |(e, item)| {
                                    let context = ct.clone();
                                    let message = format!("{}", e);
                                    let mut data = message.split(" ");
                                    if data.next() == Some("ASYNC_WAIT") {
                                        if let Some(data) = data.next() {
                                            return context.wait_for(data.to_owned()).then(
                                                move |_| Self::deserialize(item, context.clone()),
                                            );
                                        }
                                    }
                                    panic!(format!("{:?}", e))
                                })
                                .unwrap_or_else(|e| panic!(format!("{:?}", e.0)))
                        })
                        .buffer_unordered(std::usize::MAX),
                ),
                Box::pin(
                    sink.sink_map_err(|_| panic!())
                        .with::<_, _, _, ()>(|item: U::Item| ok(Self::serialize(item))),
                ),
            )),
        )
    }
}

pub enum EncodeError<T: Format, I, S: Sink<I>> {
    Format(T::Error),
    Sink(S::Error),
}

impl<I: 'static, T: Format + 'static, S: Sink<I> + 'static> Fail for EncodeError<T, I, S>
where
    T::Error: Fail,
    S::Error: Fail,
{
}

impl<T: Format, I, S: Sink<I>> Display for EncodeError<T, I, S>
where
    T::Error: Fail,
    S::Error: Fail,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            EncodeError::Format(ref err) => {
                write!(f, "Error occurred in deserialization `{}`", err)
            }
            EncodeError::Sink(ref err) => write!(f, "Error occurred in underlying sink `{}`", err),
        }
    }
}

impl<T: Format, I, S: Sink<I>> Debug for EncodeError<T, I, S>
where
    T::Error: Fail,
    S::Error: Fail,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            EncodeError::Format(ref err) => {
                write!(f, "Error occurred in deserialization `{:?}`", err)
            }
            EncodeError::Sink(ref err) => {
                write!(f, "Error occurred in underlying sink `{:?}`", err)
            }
        }
    }
}

impl<T: Format, I, S: Sink<I>> EncodeError<T, I, S> {
    fn from_sink_error(err: S::Error) -> Self {
        EncodeError::Sink(err)
    }
    fn from_format_error(err: T::Error) -> Self {
        EncodeError::Format(err)
    }
}

impl<
        'de,
        T: Format + 'static,
        C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de> + 'static + Send + Sized,
    > Encode<'de, C> for T
where
    T::Representation: Send + Clone,
    <C as Context<'de>>::Item: Send,
    <C as Sink<<C as Context<'de>>::Item>>::Error: Fail,
{
    type Output = StreamSink<
        Self::Representation,
        Self::Representation,
        EncodeError<T, <C as Context<'de>>::Item, C>,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        let (sender, receiver): (_, UnboundedReceiver<<Self as Format>::Representation>) =
            unbounded();
        let receiver = receiver
            .map(move |item: <Self as Format>::Representation| {
                let ct = ctx.clone();
                Self::deserialize(item, ctx.clone()).or_else(move |(e, item)| {
                    let context = ct.clone();
                    let message = format!("{}", e);
                    let mut data = message.split(" ");
                    if data.next() == Some("ASYNC_WAIT") {
                        if let Some(data) = data.next() {
                            return context
                                .wait_for(data.to_owned())
                                .then(move |_| Self::deserialize(item, context.clone()));
                        }
                    }
                    panic!(format!("{:?}", e))
                })
            })
            .buffer_unordered(std::usize::MAX);
        core::<dyn Executor>().unwrap().spawn(
            receiver
                .forward(sink.sink_map_err(|e| panic!(format!("{}", e))))
                .unwrap_or_else(|_| panic!()),
        );
        StreamSink(
            Box::pin(stream.map(<Self as Format>::serialize)),
            Box::pin(sender.sink_map_err(|_| panic!())),
        )
    }
}
