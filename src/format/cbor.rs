use super::Format;

use serde::{de::DeserializeSeed, Serialize};

pub struct Cbor;

impl Format for Cbor {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_cbor::to_vec(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
        context.deserialize(&mut deserializer).unwrap()
    }
}
