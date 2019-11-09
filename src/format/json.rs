use super::Format;

use serde::{de::DeserializeSeed, Serialize};

/// A format implementing JavaScript Object Notation.
///
/// JSON is a human readable object
/// serialization format used in a diverse range of applications.
/// This wraps functionality provided by the `serde_json` crate.
///
/// For this format to be used the `json` feature must be enabled.
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
    ) -> Result<T::Value, Self::Error> {
        let mut deserializer = serde_json::Deserializer::from_reader(item.as_bytes());
        context.deserialize(&mut deserializer)
    }
}
