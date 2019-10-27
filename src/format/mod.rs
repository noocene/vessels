#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "json")]
pub use json::Json;
#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "cbor")]
pub use cbor::Cbor;
#[cfg(feature = "bincode")]
pub mod bincode;
#[cfg(feature = "bincode")]
pub use bincode::Bincode;

use futures::{lazy, Future, Poll, Sink, StartSend, Stream};

use crate::{
    channel::{Context, Shim, Target},
    Kind,
};

use serde::{de::DeserializeSeed, Serialize};

use std::fmt::{Debug, Display, Formatter};

use failure::Fail;

#[doc(hidden)]
pub struct StreamSink<T: Stream, U: Sink>(pub(crate) T, pub(crate) U);

impl<T: Stream, U: Sink> Sink for StreamSink<T, U> {
    type SinkItem = U::SinkItem;
    type SinkError = U::SinkError;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.1.start_send(item)
    }
    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.1.poll_complete()
    }
}

impl<T: Stream, U: Sink> Stream for StreamSink<T, U> {
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

pub trait UniformStreamSink<T>: Sink<SinkItem = T> + Stream<Item = T> {}

impl<T, U> UniformStreamSink<T> for U where U: Sink<SinkItem = T> + Stream<Item = T> {}

pub trait Format {
    type Representation;
    type Error;

    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> Box<dyn Future<Item = T::Value, Error = Self::Error> + Send>
    where
        Self: Sized,
        T::Value: Send + 'static,
        T: Send + 'static;
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
        F::Representation: Send + 'static,
        T::Item: Send + 'static;
}

impl<'de, U, K: Kind> ApplyDecode<'de, K> for U {
    fn decode<T: Target<'de, K> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, K>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Send,
        T::Item: Send,
    {
        <F as Decode<'de, Self, K>>::decode::<T>(self)
    }
}

pub trait Decode<
    'de,
    C: UniformStreamSink<<Self as Format>::Representation> + Send + 'static,
    K: Kind,
>: Format
{
    type Output: Future<Item = K>;

    fn decode<T: Target<'de, K> + Send + 'static>(input: C) -> Self::Output
    where
        T::Item: Send;
}

pub trait Encode<'de, C: UniformStreamSink<<C as Context<'de>>::Item> + Context<'de>>:
    Format
{
    type Output: Stream<Item = <Self as Format>::Representation>
        + Sink<SinkItem = <Self as Format>::Representation>;

    fn encode(input: C) -> Self::Output;
}

impl<
        'de,
        C: UniformStreamSink<<Self as Format>::Representation> + Send + 'static,
        T: Format + 'static,
        K: Kind,
    > Decode<'de, C, K> for T
where
    Self::Representation: Send,
{
    type Output = Box<dyn Future<Item = K, Error = ()> + Send + 'de>;

    fn decode<U: Target<'de, K> + Send + 'static>(input: C) -> Self::Output
    where
        U::Item: Send,
    {
        Box::new(lazy(|| {
            let shim = U::new_shim();
            let context = shim.context();
            let (sink, stream) = input.split();
            shim.complete(StreamSink(
                stream
                    .map_err(|_| panic!())
                    .map(move |item| Self::deserialize(item, context.clone()).into_stream())
                    .flatten(),
                sink.sink_map_err(|_: <C as Sink>::SinkError| {
                    panic!();
                })
                .with(|item| Ok(Self::serialize(item)))
                .sink_map_err(|_: ()| panic!()),
            ))
            .map_err(|e| panic!(e))
        }))
    }
}

pub enum EncodeError<T: Format, S: Sink> {
    Format(T::Error),
    Sink(S::SinkError),
}

impl<T: Format + 'static, S: Sink + 'static> Fail for EncodeError<T, S>
where
    T::Error: Send + Sync + Display + Debug,
    S::SinkError: Send + Sync + Display + Debug,
{
}

impl<T: Format, S: Sink> Display for EncodeError<T, S>
where
    T::Error: Display,
    S::SinkError: Display,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            EncodeError::Format(ref err) => {
                write!(f, "Error occurred in deserialization `{}`", err)
            }
            EncodeError::Sink(ref err) => write!(f, "Error occurred in underlying sink `{}`", err),
        }
    }
}

impl<T: Format, S: Sink> Debug for EncodeError<T, S>
where
    T::Error: Debug,
    S::SinkError: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
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

impl<T: Format, S: Sink> EncodeError<T, S> {
    fn from_sink_error(err: S::SinkError) -> Self {
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
    T::Representation: Send,
    <C as Context<'de>>::Item: Send,
{
    type Output = StreamSink<
        Box<dyn Stream<Item = <Self as Format>::Representation, Error = C::Error> + Send>,
        Box<
            dyn Sink<SinkItem = <Self as Format>::Representation, SinkError = EncodeError<T, C>>
                + Send,
        >,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::new(stream.map(<Self as Format>::serialize)),
            Box::new(
                sink.sink_map_err(EncodeError::from_sink_error)
                    .with_flat_map(move |data| {
                        <Self as Format>::deserialize(data, ctx.clone())
                            .into_stream()
                            .map_err(EncodeError::from_format_error)
                    }),
            ),
        )
    }
}
