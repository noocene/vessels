use super::Format;

use serde::{de::DeserializeSeed, Serialize};

/// A format implementing the Compact Binary Object Representation.
///
/// CBOR is a binary over-the-wire format loosely based on JSON that is defined in
/// IETF RFC 7049. It is performant and concise and an effective
/// alternative to `bincode` in applications where stability of the format
/// across distinct Rust compiler versions is necessary, that of course including
/// most applications where any persistence of serialized data is intended. This functionality
/// is provided by `serde_cbor`.
///
/// For this format to be used the `cbor` feature must be enabled.
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
    ) -> Result<T::Value, Self::Error>
    where
        T::Value: Send + 'static,
        T: Send + 'static,
    {
        let mut deserializer = serde_cbor::Deserializer::from_reader(item.as_slice());
        context.deserialize(&mut deserializer)
    }
}
