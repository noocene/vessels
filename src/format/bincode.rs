use super::Format;

use serde::{de::DeserializeSeed, Serialize};

use crate::kind::Future;

/// A format implementing `bincode`.
///
/// bincode is a Rust-specific compact binary over-the-wire format with the unique guarantee of providing
/// smaller or equivalent over-the-wire size for any type as compared
/// to the size of that type in-memory. Unlike JSON and CBOR it is not self describing
/// (but therefore substantially more compact)
/// and given that it is tighly tied to the in-memory layout of Rust data structures
/// it lacks guarantees of stability or compatability across Rust versions.
/// It is, however, entirely validated and safe for untrusted input, and is very performant.
///
/// For this format to be used the `bincode` feature must be enabled.
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
    ) -> Future<Result<T::Value, (Self::Error, Self::Representation)>>
    where
        T: Sync + Send + 'static,
    {
        Box::pin(async move {
            serde_bincode::config()
                .deserialize_from_seed(context, item.as_slice())
                .map_err(|e| (e, item))
        })
    }
}
