use crate::targets;
use serde::{de::DeserializeOwned, Serialize};
use failure::Error;
use futures::Future;

pub trait SymmetricKey: Send {
    fn encrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
    fn decrypt(&self, data: &'_ [u8]) -> Box<dyn Future<Item = Vec<u8>, Error = Error> + Send>;
}

impl dyn SymmetricKey {
    pub fn new() -> impl Future<Item = Box<dyn SymmetricKey + 'static>, Error = Error> {
        #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
        targets::web::crypto::AESKey::new()
    }
}