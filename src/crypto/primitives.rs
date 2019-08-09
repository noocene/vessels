use crate::{targets, crypto};
use failure::Error;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize, ser::Serializer, de::{Visitor, Deserialize, Deserializer}};
use std::{fmt, marker::PhantomData};

/// A single-use cryptographic nonce.
///
/// Ensure that this value is not reused or security will be catastrophically compromised.
pub trait Nonce: AsRef<[u8; 12]> {
    /// Executed after encryption on ciphertext to permit nonce inclusion if necessary.
    fn after_encrypt(&self, data: &mut Vec<u8>);
}

/// A provider for single-use cryptographic nonces.
///
/// Intended for use as AEAD IVs.
pub trait NonceProvider: Send {
    /// The nonce type produced by this provider.
    type Nonce: Nonce + 'static;

    /// Generates a nonce for the next sequential encryption operation.
    fn next_encrypt(&mut self) -> Self::Nonce;
    /// Generates a nonce for the next sequential decryption operation.
    fn next_decrypt(&mut self, data: &mut Vec<u8>) -> [u8; 12];
    /// Creates a new provider.
    fn new() -> Self;
}

/// Various nonce sequence providers intended for symmetric encryption use.
pub mod nonce_providers {
    use crate::crypto::{self, primitives::{NonceProvider, Nonce}};
    use futures::Future;

    /// A randomly generated cryptographic nonce.
    #[allow(missing_copy_implementations)]
    pub struct Random;

    /// Generates cryptographically secure random nonces.
    ///
    /// NIST recommends using this technique for no more than 4 billion messages per key.
    #[allow(missing_copy_implementations)]
    pub struct RandomNonce {
        nonce: [u8; 12]
    }

    impl RandomNonce {
        fn new() -> Self {
            let mut nonce: [u8; 12] = Default::default();
            nonce.copy_from_slice(&crypto::random(12).wait().unwrap());
            RandomNonce {
                nonce
            }
        }
    }

    impl Nonce for RandomNonce {
        fn after_encrypt(&self, data: &mut Vec<u8>) {
            *data = self.nonce.as_ref().iter().copied().chain(data.iter().copied()).collect::<Vec<_>>();
        }
    }

    impl AsRef<[u8; 12]> for RandomNonce {
        fn as_ref(&self) -> &[u8; 12] { 
            &self.nonce
        }
    }

    impl NonceProvider for Random {
        type Nonce = RandomNonce;

        fn new() -> Self {
            Random
        }
        fn next_encrypt(&mut self) -> Self::Nonce {
            RandomNonce::new()
        }
        fn next_decrypt(&mut self, data: &mut Vec<u8>) -> [u8; 12] {
            let mut nonce: [u8; 12] = Default::default();
            nonce.copy_from_slice(&data[0..12]);
            *data = data.split_off(12);
            nonce
        }
    }
}

/// Backed by AES-128 in GCM on all platforms, interoperable.
/// 
/// AES-256 is not used as the probability of success for a brute-force attack on AES-128 is already far more slim than necessary and the AES-256 key schedule is less well designed.
pub trait SymmetricKey<T>: Send {
    /// Encrypts and signs the provided data.
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    /// Decrypts and authenticates the provided data.
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    /// Exports a key as a raw 128-bit byte array.
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error>>;
}

impl<T: NonceProvider + 'static> dyn SymmetricKey<T> {
    /// Constructs a new random key from a secure source of entropy.
    pub fn new() -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::primitives::AESKey::new()
    }
    /// Imports a key from a raw 128-bit byte array.
    pub fn from_bytes(data: [u8; 16]) -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::primitives::AESKey::from_bytes(data)
    }
}

impl<T> Serialize for Box<dyn SymmetricKey<T>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.as_bytes().wait().unwrap())
    }
}

struct SymmetricKeyVisitor<T: NonceProvider + 'static>(PhantomData<T>);

impl<'de, T: NonceProvider + 'static> Visitor<'de> for SymmetricKeyVisitor<T> {
    type Value = Box<dyn SymmetricKey<T>>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { 
        formatter.write_str("raw binary AES key data")
    }
    fn visit_bytes<E>(self, v: &'_ [u8]) -> Result<Self::Value, E> {
        let mut a: [u8; 16] = Default::default();
        a.copy_from_slice(v);
        Ok(SymmetricKey::from_bytes(a).wait().unwrap())
    }
}

impl<'de, T: NonceProvider + 'static> Deserialize<'de> for Box<dyn SymmetricKey<T>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(SymmetricKeyVisitor(PhantomData))
    }
}