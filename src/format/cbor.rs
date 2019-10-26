use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use futures::{lazy, sync::oneshot::channel, Future};

pub struct Cbor;

impl Format for Cbor {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_cbor::to_vec(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> Box<dyn Future<Item = T::Value, Error = ()> + Send>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        Box::new(lazy(move || {
            let (sender, receiver) = channel();
            std::thread::spawn(move || {
                let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
                context
                    .deserialize(&mut deserializer)
                    .and_then(|item| sender.send(item).map_err(|_| panic!()))
                    .map_err(|_| panic!())
            });
            receiver.map_err(|_| panic!())
        }))
    }
}
