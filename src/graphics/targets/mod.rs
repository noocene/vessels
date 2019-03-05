#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub mod web;
