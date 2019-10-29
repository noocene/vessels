use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    channel::oneshot::{channel, Receiver},
    executor::ThreadPool,
    future::{lazy, BoxFuture},
    TryFutureExt,
};

pub struct Cbor;

impl Format for Cbor {
    type Representation = Vec<u8>;
    type Error = serde_cbor::Error;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_cbor::to_vec(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> BoxFuture<'static, Result<T::Value, Self::Error>>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        let (sender, receiver): (_, Receiver<Result<T::Value, Self::Error>>) = channel();
        ThreadPool::new().unwrap().spawn_ok(lazy(move |_| {
            let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
            sender
                .send(context.deserialize(&mut deserializer))
                .unwrap_or_else(|e| panic!(e))
        }));
        Box::pin(receiver.unwrap_or_else(|e| panic!(e)))
    }
}
