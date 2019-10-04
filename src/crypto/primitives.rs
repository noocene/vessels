use crate::targets;
use failure::Error;
use futures::Future;
use serde::{
    de::{Deserialize, Deserializer, Visitor},
    ser::Serializer,
    Serialize,
};
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
pub trait NonceProvider {
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
    use crate::crypto::{
        self,
        primitives::{Nonce, NonceProvider},
    };
    use futures::Future;

    /// A randomly generated cryptographic nonce.
    #[allow(missing_copy_implementations)]
    pub struct Random;

    /// Generates cryptographically secure random nonces.
    ///
    /// NIST recommends using this technique for no more than 4 billion messages per key.
    #[allow(missing_copy_implementations)]
    pub struct RandomNonce {
        nonce: [u8; 12],
    }

    impl RandomNonce {
        fn new() -> Self {
            let mut nonce: [u8; 12] = Default::default();
            nonce.copy_from_slice(&crypto::random(12).wait().unwrap());
            RandomNonce { nonce }
        }
    }

    impl Nonce for RandomNonce {
        fn after_encrypt(&self, data: &mut Vec<u8>) {
            *data = self
                .nonce
                .as_ref()
                .iter()
                .copied()
                .chain(data.iter().copied())
                .collect::<Vec<_>>();
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

/// Symmetric encryption backed by AES-128 in GCM on all platforms, interoperable.
///
/// AES-256 is not used as the probability of success for a brute-force attack on AES-128 is already far more slim than necessary and the AES-256 key schedule is less well designed.
pub trait SymmetricKey<T> {
    /// Encrypts and signs the provided data.
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
    /// Decrypts and authenticates the provided data.
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
    /// Exports a key as a raw 128-bit byte array.
    fn as_bytes(&self) -> Box<dyn Future<Item = [u8; 16], Error = Error>>;
}

impl<T: NonceProvider + 'static> dyn SymmetricKey<T> {
    /// Constructs a new random key from a secure source of entropy.
    pub fn new() -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::crypto::primitives::AESKey::new();
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        return targets::native::crypto::primitives::AESKey::new();
    }
    /// Imports a key from a raw 128-bit byte array.
    pub fn from_bytes(
        data: [u8; 16],
    ) -> impl Future<Item = Box<dyn SymmetricKey<T> + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::crypto::primitives::AESKey::from_bytes(data);
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        return targets::native::crypto::primitives::AESKey::from_bytes(data);
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

/// Private key for cryptographic signing.
///
/// Be careful with this. Having it compromised/disseminated in plaintext is generally a pretty bad idea in almost any conceivable cryptosystem.
pub trait SigningKey {
    /// Signs the provided data guaranteeing authenticity and integrity.
    fn sign(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
    /// Exports the underlying key-pair in PKCS#11 compatible form.
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
}

/// Public key for cryptographic signature verification.
pub trait VerifyingKey {
    /// Verifies the provided data for integrity and authenticity using the provided signature.
    fn verify(
        &self,
        data: &'_ [u8],
        signature: &'_ [u8],
    ) -> Box<dyn Future<Item = bool, Error = Error>>;
    /// Exports the raw public key.
    fn as_bytes(&self) -> Box<dyn Future<Item = Vec<u8>, Error = Error>>;
}

/// Asymmetric cryptographic signatures backed by ECDSA using NIST P-256 curve and SHA-256 for hashing.
///
/// Trusting P-256? Well, we'd hope the tech backing our abstractions (SubtleCrypto and therefore the user's browser, `ring` and therefore BoringSSL essentially and therefore Chrome) is a solid implementation. With regards to backdoors or
/// "cooked" seeding if you're up against an entity with the resources to backdoor everyone's crypto in such a way that 15+ years of cryptanalysis can't figure out how it's been done your signature algorithm is probably the least of your concerns.
pub trait SigningKeyPair {}

impl dyn SigningKeyPair {
    /// Creates a new key pair.
    pub fn new() -> impl Future<
        Item = (
            Box<dyn SigningKey + 'static>,
            Box<dyn VerifyingKey + 'static>,
        ),
        Error = Error,
    > {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::crypto::primitives::ECDSAKeyPair::new();
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        return targets::native::crypto::primitives::ECDSAKeyPair::new();
    }
}

impl dyn VerifyingKey {
    /// Imports a public key for signature verification from the raw key bytes.
    pub fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn VerifyingKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::crypto::primitives::ECDSAPublicKey::from_bytes(data);
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        return targets::native::crypto::primitives::ECDSAPublicKey::from_bytes(data);
    }
}

impl dyn SigningKey {
    /// Imports a key pair for cryptographic signing from a PKCS#8 compatible byte slice.
    pub fn from_bytes(
        data: &'_ [u8],
    ) -> impl Future<Item = Box<dyn SigningKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        return targets::web::crypto::primitives::ECDSAPrivateKey::from_bytes(data);
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        return targets::native::crypto::primitives::ECDSAPrivateKey::from_bytes(data);
    }
}

struct SigningKeyVisitor;

impl<'de> Visitor<'de> for SigningKeyVisitor {
    type Value = Box<dyn SigningKey>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("pkcs#8 compatible ECDSA keypair data")
    }
    fn visit_bytes<E>(self, v: &'_ [u8]) -> Result<Self::Value, E> {
        Ok(SigningKey::from_bytes(v).wait().unwrap())
    }
}

impl<'de> Deserialize<'de> for Box<dyn SigningKey> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(SigningKeyVisitor)
    }
}

struct VerifyingKeyVisitor;

impl<'de> Visitor<'de> for VerifyingKeyVisitor {
    type Value = Box<dyn VerifyingKey>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("pkcs#8 compatible ECDSA keypair data")
    }
    fn visit_bytes<E>(self, v: &'_ [u8]) -> Result<Self::Value, E> {
        Ok(VerifyingKey::from_bytes(v).wait().unwrap())
    }
}

impl<'de> Deserialize<'de> for Box<dyn VerifyingKey> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(VerifyingKeyVisitor)
    }
}

impl Serialize for Box<dyn SigningKey> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.as_bytes().wait().unwrap())
    }
}

impl Serialize for Box<dyn VerifyingKey> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.as_bytes().wait().unwrap())
    }
}
