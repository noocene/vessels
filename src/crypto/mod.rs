use failure::Error;
use crate::targets;
use futures::Future;

/// Abstracted cryptographic primitives.
/// It is not recommended to use these primitives directly, instead use a pre-defined cryptosystem.
pub mod primitives;

/// Generates `bytes` random bytes using a secure source of entropy.
pub fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::crypto::random(bytes)
}