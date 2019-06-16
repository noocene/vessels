#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
pub(crate) mod native;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub(crate) mod web;
