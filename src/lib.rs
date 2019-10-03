//! vessels is a software development framework intended to facilitate
//! the ergonomic development of performant cross-platform informational
//! ecosystems using a novel paradigm based on a single-source distributed
//! object model and low-overhead containerisation.

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

/// Provides abstracted cryptographic primitives.
pub mod crypto;
/// Provides a cross-platform abstracted asynchronous executor.
pub mod executor;
/// Provides interfaces to graphics APIs.
pub mod graphics;
/// Provides interfaces to user input/HID APIs.
pub mod input;
/// Provides interfaces to abstracted networking APIs.
pub mod network;
/// Provides functionality for generating APIs and other RPC protocols.
pub mod protocol;
mod targets;
mod util;
