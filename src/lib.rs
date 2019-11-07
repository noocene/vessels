#[macro_use]
extern crate erased_serde;

extern crate self as vessels;

pub mod channel;
#[doc(inline)]
pub use channel::OnTo;
use channel::{Channel, Target};
pub mod format;
#[doc(inline)]
pub use format::{ApplyDecode, ApplyEncode};
pub mod core;
pub mod kind;
#[doc(inline)]
pub use crate::core::core;
pub mod reflection;

use downcast_rs::{impl_downcast, Downcast};
use erased_serde::Serialize as ErasedSerialize;
use failure::Fail;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::any::Any;

/// Generates an implementation of `Kind` for trait objects.
///
/// Annotating an object-safe trait with this macro will allow the use of
/// trait objects constructed from that trait as `Kind`s. This uses the implementations
/// of `Kind` for boxed `dyn Fn`s internally and therefore only functions that return
/// `Future` or `Stream` will result in an implementation that compiles. This is intended,
/// synchronous returns in an RPC system are an antipattern and this system avoids them.
/// ```
/// use vessels::object;
///
/// #[object]
/// pub trait Object<T: Kind> {
///     fn test(&self) -> Future<T>;
/// }
/// ```
/// The above will generate an implementation of Kind for `Box<dyn Object<T>>` where `T: Kind`.
/// Generic parameters are, as thereby evidenced, supported. Functions with between zero and sixteen arguments
/// not including receiver are supported where all arguments implement `Kind`. Annotated wrapper Kinds as with
/// the primary derive macro are not supported, though support is planned.
///
/// Associated type parameters are not permitted, they offer no advantage on trait objects as they must be
/// statically described therein. Moreover, they would require additional parametrization of `Trait` which would
/// come at an ergonomics cost without any benefit.
pub use derive::object;

/// Generates an implementation of `Kind` for a struct or enum.
///
/// This macro has a number of modes of operation.
/// First, it may be used in a fashion equivalent to the manner of operation of standard library derive macros.
/// ```
/// use vessels::Kind;
///
/// #[derive(Kind)]
/// struct Person<T> {
///     name: T,
///     say_hello: Box<dyn Fn() -> T + Sync + Send>,
/// }
/// ```
/// This will generate an implementation of `Kind` for the annotated type given an extant implementation of `Kind`
/// for each field of that type. There is further nuance to this mode of operation, but to explain it is best to first
/// demonstrate the other primary manner of operation.
/// ```
/// use vessels::{Kind, kind::using};
/// use serde::{Serialize, Deserialize}
///
/// #[derive(Serialize, Deserialize)]
/// struct NotKind;
///
/// #[derive(Serialize, Deserialize, Kind)]
/// #[kind(using::Serde)]
/// struct Person {
///     name: String,
///     data: NotKind,
/// }
/// ```
/// This will generate an implementation of `Kind` for the annotated type despite `NotKind` lacking a valid implementation.
/// The types, provided in `vessels::kind::using`, that provide `AsKind` trait implementations, allow for the use of an
/// alternative bijection for structs and enums that implement some certain traits permitting such a thing. To finally attend
/// to the additional mode of operation mentioned earlier, these `#[kind()]` annotations may be used with the initially discussed
/// syntax.
/// ```
/// #[derive(Kind)]
/// struct Person {
///     name: String,
///     #[kind(using::Serde)]
///     data: NotKind,
/// }
/// ```
/// Annotating a field of a struct or enum with `#[kind()]`, if the type provided in the attribute annotation is a valid `AsKind`
/// for the type of that field, will cause the overarching derivation to use that type as a wrapper to produce a valid `Kind` bijection.
/// All of the described behavior also functions for arbitrary generic parameters and when used in enums with both named and unnamed fields.
/// ```
/// #[derive(Kind)]
/// enum Entity<T: Kind> {
///     Person {
///         name: String,
///         #[kind(using::Serde)]
///         data: NotKind,
///     },
///     UnnamedFields(#[kind(using::Serde)] NotKind, T)
/// }
/// ```
pub use derive::Kind;

/// Generates the entry point of a vessel.
///
/// ```
/// use vessels::export;
///
/// export! {
///     "test".to_owned()
/// }
/// ```
///
/// `export` wraps a block that returns a `Kind` and generates the entry point for a vessel providing that `Kind`.
/// `export` should be used in a `bin` target of a crate i.e. for the default cargo configuration `main.rs`.
/// No `main` function is necessary when `export` is used, in fact the presence of a `main` function will cause an
/// exported vessel to fail to compile due to a symbol conflict. Finally, `export` cannot be used on non-wasm
/// targets or on wasm when the `core` feature is enabled, as neither of those compilation states are valid
/// for the compilation of a vessel.
pub use derive::export;

#[doc(hidden)]
pub use futures;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use void;

/// The result of reconstructing a Kind.
pub type ConstructResult<K> = Result<K, <K as Kind>::ConstructError>;
/// The result of deconstructing a Kind.
pub type DeconstructResult<K> = Result<(), <K as Kind>::DeconstructError>;

/// A type with a bijection to over-the-wire data.
///
/// Kind is an advanced distributed object or RPC system that permits the over-the-wire serialization
/// and deserialization of an implicitly flattened version of the complex nested structures required
/// to produce a full type-level isomorphic representation of arbitrary composed data types.
///
/// Vessels provides `Kind` implementations for many primitive types from the standard library as
/// well as futures, streams, a variety of boxed function types, and more.
///
/// Vessels also provides a derive macro that automatically generates `Kind` implementations for
/// structs and enums in addition to the `object` macro for generating `Kind` implementations for
/// trait objects of user-defined traits.
///
/// Authors of third-party crates are encouraged to derive or implement Kind or Kind providers for
/// types their crates expose that might be useful over some form of wire boundary, be it network, IPC,
/// or any other similar transport.
pub trait Kind: Any + Sized + Send + 'static {
    /// The item transmitted over the network **to** the construction task
    /// from deconstruction.
    type ConstructItem: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static;
    /// The failure condition of constructing a concrete type from communicated data.
    type ConstructError: Fail;
    /// The concrete future type returned by the construction process.
    type ConstructFuture: Future<Output = ConstructResult<Self>> + Send + 'static;

    /// Constructs the `Kind` from the provided channel. This method should return
    /// immediately and, if necessary, move `channel` into some shim structure,
    /// async block, or other owned state specified by `ConstructFuture`.
    fn construct<C: Channel<Self::ConstructItem, Self::DeconstructItem>>(
        channel: C,
    ) -> Self::ConstructFuture;

    /// The item transmitted over the network **from** the construction task
    /// to deconstruction.
    type DeconstructItem: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static;
    /// The failure condition of constructing a concrete type from communicated data.
    type DeconstructError: Fail;
    /// The concrete future type returned by the deconstruction process. This is
    /// used to only to communicate failure of deconstruction and does not return
    /// a value.
    type DeconstructFuture: Future<Output = DeconstructResult<Self>> + Send + 'static;

    /// Moves out of the `Kind` and deconstructs on to the provided channel.
    /// As with `construct`, this method should return immediately.
    fn deconstruct<C: Channel<Self::DeconstructItem, Self::ConstructItem>>(
        self,
        channel: C,
    ) -> Self::DeconstructFuture;
}

/// An erased representation of any serializable type used in communication
/// by `Kind`.
pub(crate) trait SerdeAny: erased_serde::Serialize + Downcast + Send {}

impl_downcast!(SerdeAny);

serialize_trait_object!(SerdeAny);

impl<T: ?Sized> SerdeAny for T where T: ErasedSerialize + Downcast + Send {}
