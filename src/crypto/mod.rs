use crate::targets;
use failure::Error;
use futures::Future;

/// Abstracted cryptographic primitives.
///
/// It is not recommended to use these primitives directly, instead use a pre-defined cryptosystem.
pub mod primitives;

/// Generates random bytes using a secure source of entropy.
pub fn random(bytes: u32) -> impl Future<Item = Vec<u8>, Error = Error> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::crypto::random(bytes);
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    return targets::native::crypto::random(bytes);
}
