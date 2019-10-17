pub mod as_bytes;
pub use as_bytes::AsBytes;
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
    Value,
};

use serde::{de::DeserializeSeed, Serialize};

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

pub trait ApplyDecode<'de, V: Value> {
    fn decode<T: Target<'de, V> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, V>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Send;
}

impl<'de, U, V: Value> ApplyDecode<'de, V> for U {
    fn decode<T: Target<'de, V> + Send + 'static, F: Format + 'static>(
        self,
    ) -> <F as Decode<'de, Self, V>>::Output
    where
        Self: UniformStreamSink<F::Representation> + Send + Sized + 'static,
        F::Representation: Send,
    {
        <F as Decode<'de, Self, V>>::decode::<T>(self)
    }
}

pub trait Decode<
    'de,
    C: UniformStreamSink<<Self as Format>::Representation> + Send + 'static,
    V: Value,
>: Format
{
    type Output: Future<Item = V>;

    fn decode<T: Target<'de, V> + Send + 'static>(input: C) -> Self::Output;
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
        V: Value,
    > Decode<'de, C, V> for T
where
    Self::Representation: Send,
{
    type Output = Box<dyn Future<Item = V, Error = ()> + Send + 'de>;

    fn decode<U: Target<'de, V> + Send + 'static>(input: C) -> Self::Output {
        Box::new(lazy(|| {
            let shim = U::new_shim();
            let context = shim.context();
            let (sink, stream) = input.split();
            shim.complete(StreamSink(
                stream.map(move |item| Self::deserialize(item, context.clone())),
                sink.sink_map_err(|k: <C as Sink>::SinkError| {
                    panic!();
                    ()
                })
                .with(|item| Ok(Self::serialize(item)))
                .sink_map_err(|_: ()| ()),
            ))
            .map_err(|e| panic!(e))
        }))
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
