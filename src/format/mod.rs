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
    future::{ok, BoxFuture},
    stream::BoxStream,
    task::Context as FContext,
    Future, FutureExt, Poll, Sink, SinkExt, Stream, StreamExt, TryFutureExt,
};

use crate::{
    channel::{Context, Shim, Target, Waiter},
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
/// Because the implementation of DeserializeSeed for `IdChannel`, the current
/// principal transport format, blocks until type information is available
/// it is necessary for `Format`s to handle the execution of this synchronous
/// blocking operation in the context of another thread and provide a future of
/// its completion. It is recognized that this is less than ideal for a number
/// of reasons, chief among them the extreme performance cost of the current technique
/// of spawning a proliferation of OS threads (due to the inability to block on a future
/// on threads that have a currently running threaded executor), another key reason being
/// the inavailability of threading or any sort of yield/wake scheduling in most browser
/// environments.
///
/// The limiting factor in changing the method in which this deserialization occurs is the
/// inability, due to the manner in which the `DeserializeSeed` trait is defined, to place
/// additional bounds on the `Deserializer` used i.e. to require a Send bound or longer lifetime
/// necessary to move the deserialization properly into an asynchronous context in and of itself i.e.
/// to deserialize directly into a future.
pub trait Format {
    /// The underlying representation used by this `Format`, i.e. `Vec<u8>` for most
    /// binary formats and `String` for those of a human-readable nature.
    type Representation;
    /// The failure condition of this format. This may be encountered during deserialization.
    type Error: Fail;

    /// Serializes the provided item. This operation is synchronous.
    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    /// Deserializes an item from the provided formatted representation.
    /// This operation is currently asynchronous and requires the duplication of source for
    /// a substantially less-than-ideal thread scheduling system, as discussed above.
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> BoxFuture<'static, Result<T::Value, Self::Error>>
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
        <Self as Sink<F::Representation>>::Error: Send,
        T::Item: Send + 'static;
}

impl<'de, U, K: Kind> ApplyDecode<'de, K> for U {
    fn decode<T: Target<'de, K> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Clone + Send,
        <Self as Sink<F::Representation>>::Error: Send,
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
    <C as Sink<<Self as Format>::Representation>>::Error: Send,
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
                            Self::deserialize(item.clone(), context.clone())
                                .or_else(move |e| {
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
                                    panic!(e)
                                })
                                .unwrap_or_else(|e| panic!(format!("{:?}", e)))
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
    T::Error: Send + Sync + Display + Debug,
    S::Error: Send + Sync + Display + Debug,
{
}

impl<T: Format, I, S: Sink<I>> Display for EncodeError<T, I, S>
where
    T::Error: Display,
    S::Error: Display,
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
    T::Error: Debug,
    S::Error: Debug,
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
    <C as Sink<<C as Context<'de>>::Item>>::Error: Send,
{
    type Output = StreamSink<
        Self::Representation,
        Self::Representation,
        EncodeError<T, <C as Context<'de>>::Item, C>,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::pin(stream.map(<Self as Format>::serialize)),
            Box::pin(
                sink.sink_map_err(EncodeError::from_sink_error)
                    .with_flat_map(move |item: <Self as Format>::Representation| {
                        let ctx = ctx.clone();
                        Self::deserialize(item.clone(), ctx.clone())
                            .or_else(move |e| {
                                let context = ctx.clone();
                                let message = format!("{}", e);
                                let mut data = message.split(" ");
                                if data.next() == Some("ASYNC_WAIT") {
                                    if let Some(data) = data.next() {
                                        return context.wait_for(data.to_owned()).then(move |_| {
                                            Self::deserialize(item, context.clone())
                                        });
                                    }
                                }
                                panic!(e)
                            })
                            .map_err(EncodeError::from_format_error)
                            .into_stream()
                    }),
            ),
        )
    }
}
