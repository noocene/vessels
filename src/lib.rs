//! Vitruvia is a general-purpose hardware abstraction layer that aims to provide a
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

mod errors;
/// Provides interfaces to 2D graphics APIs.
pub mod graphics_2d;
/// Provides interfaces to user interaction/HID APIs.
pub mod interaction;
/// Provides helper types that allow ergonomic construction and styling of 2D vector graphics.
pub mod path;
mod targets;
/// Contains types to help represent and construct styled text.
pub mod text;
mod util;
