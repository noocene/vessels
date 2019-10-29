use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    channel::oneshot::{channel, Receiver},
    executor::ThreadPool,
    future::{lazy, BoxFuture},
    TryFutureExt,
};

pub struct Bincode;

impl Format for Bincode {
    type Representation = Vec<u8>;
    type Error = serde_bincode::Error;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_bincode::serialize(&item).unwrap()
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
            sender
                .send(serde_bincode::config().deserialize_from_seed(context, item.as_slice()))
                .unwrap_or_else(|e| panic!(e))
        }));
        Box::pin(receiver.unwrap_or_else(|e| panic!(e)))
    }
}
