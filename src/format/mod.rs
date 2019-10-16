pub mod as_bytes;
pub use as_bytes::AsBytes;
pub mod json;
pub use json::Json;
pub mod cbor;
pub use cbor::Cbor;

use futures::{Poll, Sink, StartSend, Stream};

use crate::channel::Context;

use serde::{de::DeserializeSeed, Serialize};

#[doc(hidden)]
pub struct StreamSink<T: Stream, U: Sink>(T, U);

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

    fn serialize<T: Serialize>(item: T) -> Self::Representation
    where
        Self: Sized;
    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value
    where
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

pub trait ApplyDecode<'de> {
    fn decode<F: Format + Decode<'de, Self>>(self) -> <F as Decode<'de, Self>>::Output
    where
        Self: Sized + UniformStreamSink<<F as Format>::Representation> + Context<'de>;
}

impl<'de, T> ApplyDecode<'de> for T {
    fn decode<F: Format + Decode<'de, Self>>(self) -> <F as Decode<'de, Self>>::Output
    where
        Self: Sized + UniformStreamSink<<F as Format>::Representation> + Context<'de>,
    {
        <F as Decode<_>>::decode(self)
    }
}

pub trait Decode<'de, C: UniformStreamSink<<Self as Format>::Representation> + Context<'de>>:
    Format
{
    type Output: Stream<Item = <C as Context<'de>>::Item>
        + Sink<SinkItem = <C as Context<'de>>::Item>;

    fn decode(input: C) -> Self::Output;
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
        T: Format + 'static,
        C: UniformStreamSink<<Self as Format>::Representation> + Context<'de> + 'static + Send,
    > Decode<'de, C> for T
where
    Self::Representation: Send,
{
    type Output = StreamSink<
        Box<dyn Stream<Item = <C as Context<'de>>::Item, Error = ()> + Send>,
        Box<dyn Sink<SinkItem = <C as Context<'de>>::Item, SinkError = ()> + Send>,
    >;

    fn decode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::new(
                stream
                    .map_err(|_| ())
                    .map(move |data| <Self as Format>::deserialize(data, ctx.clone())),
            ),
            Box::new(
                sink.sink_map_err(|_| ())
                    .with(|data| Ok(<Self as Format>::serialize(data))),
            ),
        )
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
        Box<dyn Stream<Item = <Self as Format>::Representation, Error = ()> + Send>,
        Box<dyn Sink<SinkItem = <Self as Format>::Representation, SinkError = ()> + Send>,
    >;

    fn encode(input: C) -> Self::Output {
        let ctx = input.context();
        let (sink, stream) = input.split();
        StreamSink(
            Box::new(stream.map_err(|_| ()).map(<Self as Format>::serialize)),
            Box::new(
                sink.sink_map_err(|_| ())
                    .with(move |data| Ok(<Self as Format>::deserialize(data, ctx.clone()))),
            ),
        )
    }
}
