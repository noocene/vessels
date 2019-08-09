use crate::targets;
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize, ser::Serializer, de::{Visitor, Deserialize, Deserializer}};
use std::fmt;

/// Backed by AES-128 in GCM on all platforms, interoperable.
/// 
/// AES-256 is not used as the probability of success for a brute-force attack on AES-128 is already far more slim than necessary and the AES-256 key schedule is less well designed.
pub trait SymmetricKey: Send {
    /// Encrypts and signs the provided data.
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    /// Decrypts and authenticates the provided data.
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    /// Exports a key as a raw 128-bit byte array.
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error>>;
}

impl dyn SymmetricKey {
    /// Constructs a new random key from a secure source of entropy.
    pub fn new() -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::primitives::AESKey::new()
    }
    /// Imports a key from a raw 128-bit byte array.
    pub fn from_bytes(data: [u8; 16]) -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
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
    fn visit_bytes<E>(self, v: &'_ [u8]) -> Result<Self::Value, E> {
        let mut a: [u8; 16] = Default::default();
        a.copy_from_slice(v);
        Ok(SymmetricKey::from_bytes(a).wait().unwrap())
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