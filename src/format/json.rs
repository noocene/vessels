use super::Format;

use serde::{de::DeserializeSeed, Serialize};

pub struct Json;

impl Format for Json {
    type Representation = String;

    fn serialize<T: Serialize>(item: T) -> Self::Representation {
        serde_json::to_string(&item).unwrap()
    }

    fn deserialize<'de, T: DeserializeSeed<'de>>(
        item: Self::Representation,
        context: T,
    ) -> T::Value {
        let mut deserializer = serde_json::Deserializer::from_reader(item.as_bytes());
        context.deserialize(&mut deserializer).unwrap()
    }
}
