use crate::{
    core::{data::Checksum, UnimplementedError},
    kind::Future,
    object,
};

use serde::{de::DeserializeOwned, Serialize};

pub trait HashData {
    fn hash_data<T: Serialize + DeserializeOwned>(&self, data: &T) -> Future<Checksum>;
}

impl<T: Hasher> HashData for T {
    fn hash_data<D: Serialize + DeserializeOwned>(&self, data: &D) -> Future<Checksum> {
        self.hash(serde_cbor::to_vec(&data).unwrap())
    }
}

impl HashData for Box<dyn Hasher> {
    fn hash_data<T: Serialize + DeserializeOwned>(&self, data: &T) -> Future<Checksum> {
        self.hash(serde_cbor::to_vec(&data).unwrap())
    }
}

#[object]
pub trait Hasher {
    fn hash(&self, data: Vec<u8>) -> Future<Checksum>;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn Hasher {
    pub fn new() -> Result<Box<dyn Hasher>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Hasher::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Hasher::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "random number generation".to_owned(),
        });
    }
}
