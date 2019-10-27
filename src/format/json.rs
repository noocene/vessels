use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{
    lazy,
    sync::oneshot::{channel, Receiver},
    Future,
};

pub struct Json;

impl Format for Json {
    type Representation = String;
    type Error = serde_json::Error;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_json::to_string(&item).unwrap()
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
                let mut deserializer = serde_json::Deserializer::from_reader(item.as_bytes());
                sender
                    .send(context.deserialize(&mut deserializer))
                    .map_err(|e| panic!(e))
            }));
            receiver.map_err(|e| panic!(e)).and_then(|item| item)
        }))
    }
}
