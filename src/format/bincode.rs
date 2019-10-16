use super::Format;

use serde::{de::DeserializeSeed, Serialize};

pub struct Bincode;

impl Format for Bincode {
    type Representation = Vec<u8>;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_bincode::serialize(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        serde_bincode::config()
            .deserialize_from_seed(context, item.as_slice())
            .unwrap()
    }
}
