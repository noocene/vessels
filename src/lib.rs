//! vessels is a general-purpose hardware abstraction layer that aims to provide a
//! consistent and ergonomic set of interfaces for common platform-specific APIs.

#![warn(
    missing_copy_implementations,
    anonymous_parameters,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![recursion_limit = "128"]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[macro_use]
extern crate stdweb;

#[cfg(any(target_arch = "linux", target_arch = "macos", target_arch = "linux"))]
extern crate cairo;
#[cfg(any(target_arch = "linux", target_arch = "macos", target_arch = "linux"))]
extern crate cairo_sys;
#[cfg(any(target_arch = "linux", target_arch = "macos", target_arch = "linux"))]
extern crate gl;
#[cfg(any(target_arch = "linux", target_arch = "macos", target_arch = "linux"))]
extern crate glutin;

mod errors;
/// Provides a cross-platform abstracted asynchronous executor.
pub mod executor;
/// Provides interfaces to 2D graphics APIs.
pub mod graphics;
/// Provides interfaces to user input/HID APIs.
pub mod input;
mod targets;
mod util;
