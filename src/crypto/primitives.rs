use crate::targets;
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize, ser::Serializer, de::{Visitor, Deserialize, Deserializer}};
use std::fmt;

/// Backed by AES-128 in GCM on all platforms, interoperable. AES-256 is not used as the probability of success for a brute-force attack on AES-128 is already far more slim than necessary and the AES-256 key schedule is less well designed.
pub trait SymmetricKey: Send {
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
}

impl dyn SymmetricKey {
    pub fn new() -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::primitives::AESKey::new()
    }
    pub fn from_bytes(data: &'_ [u8]) -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::primitives::AESKey::from_bytes(data)
    }
}

impl Serialize for Box<dyn SymmetricKey> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.as_bytes().wait().unwrap())
    }
}

struct SymmetricKeyVisitor;

impl<'de> Visitor<'de> for SymmetricKeyVisitor {
    type Value = Box<dyn SymmetricKey>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { 
        formatter.write_str("raw binary AES key data")
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> {
        Ok(SymmetricKey::from_bytes(v).wait().unwrap())
    }
}

impl<'de> Deserialize<'de> for Box<dyn SymmetricKey> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(SymmetricKeyVisitor)
    }
}