use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    lazy,
    sync::oneshot::{channel, Receiver},
    Future,
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
    ) -> Box<dyn Future<Item = T::Value, Error = Self::Error> + Send>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        Box::new(lazy(move || {
            let (sender, receiver): (_, Receiver<Result<T::Value, Self::Error>>) = channel();
            tokio::spawn(lazy(move || {
                let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
                sender
                    .send(context.deserialize(&mut deserializer))
                    .map_err(|e| panic!(e))
            }));
            receiver.map_err(|e| panic!(e)).and_then(|item| item)
        }))
    }
}
