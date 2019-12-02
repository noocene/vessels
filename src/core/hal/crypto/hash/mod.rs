use crate::{
    core::{data::Checksum, UnimplementedError},
    kind::Future,
    object,
};

#[object]
pub trait Hash {
    fn hash(&self, data: Vec<u8>) -> Future<Checksum>;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn Hash {
    pub fn new() -> Result<Box<dyn Hash>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Hash::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Hash::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "random number generation".to_owned(),
        });
    }
}
