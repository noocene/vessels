#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate stdweb;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate stdweb_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate weak_table;

pub mod graphics;
pub mod util;
