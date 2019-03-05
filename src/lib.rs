#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
#[macro_use]
extern crate stdweb;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate stdweb_derive;

pub mod graphics;
mod util;
