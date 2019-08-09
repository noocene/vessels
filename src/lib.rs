//! Vessels is an application development framework intended to facilitate
//! the ergonomic development of performant cross-platform applications using
//! a novel paradigm based on a single-source distributed object model and
//! low-overhead containerisation.

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

#[allow(unused_imports)]
#[macro_use]
extern crate vessels_derive;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[macro_use]
extern crate lazy_static;

/// Provides abstracted cryptographic primitives.
pub mod crypto;
mod errors;
/// Provides a cross-platform abstracted asynchronous executor.
pub mod executor;
/// Provides interfaces to 2D graphics APIs.
pub mod graphics_2d;
/// Provides interfaces to user interaction/HID APIs.
pub mod interaction;
/// Provides interfaces to abstracted networking APIs.
pub mod network;
/// Provides helper types that allow ergonomic construction and styling of 2D vector graphics.
pub mod path;
/// Provides functionality for generating APIs and other RPC protocols.
pub mod protocol;
mod targets;
/// Provides types to help represent and construct styled text.
pub mod text;
mod util;
