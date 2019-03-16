#![recursion_limit = "128"]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[macro_use]
extern crate stdweb;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb_derive;

extern crate failure;

mod errors;
pub mod graphics_2d;
pub mod input;
pub mod path;
mod targets;
pub mod text;
mod util;
