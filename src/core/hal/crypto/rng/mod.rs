use crate::{core::UnimplementedError, kind::Infallible, object};

#[object]
pub trait Rng {
    fn bytes(&mut self, len: usize) -> Infallible<Vec<u8>>;
}

#[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
mod native;
#[cfg(all(target_arch = "wasm32", feature = "core"))]
mod web;

impl dyn Rng {
    pub fn new() -> Result<Box<dyn Rng>, UnimplementedError> {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        return Ok(web::Rng::new());
        #[cfg(all(not(target_arch = "wasm32"), feature = "core"))]
        return Ok(native::Rng::new());
        #[cfg(not(feature = "core"))]
        return Err(UnimplementedError {
            feature: "Random number generation".to_owned(),
        });
    }
}
