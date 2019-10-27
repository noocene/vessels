use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    lazy,
    sync::oneshot::{channel, Receiver},
    Future,
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
    ) -> Box<dyn Future<Item = T::Value, Error = Self::Error> + Send>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        Box::new(lazy(move || {
            let (sender, receiver): (_, Receiver<Result<T::Value, Self::Error>>) = channel();
            tokio::spawn(lazy(move || {
                sender
                    .send(serde_bincode::config().deserialize_from_seed(context, item.as_slice()))
                    .map_err(|e| panic!(e))
            }));
            receiver.map_err(|e| panic!(e)).and_then(|item| item)
        }))
    }
}
