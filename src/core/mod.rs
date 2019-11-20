use failure::Fail;
use std::{
    any::{Any, TypeId},
    fmt::{self, Display, Formatter},
};

use crate::kind::Future;

pub mod executor;
pub use executor::Executor;

pub mod hal;
pub mod orchestrator;

pub type Vessel<T> = Box<dyn FnOnce() -> Future<T> + Send + Sync>;

pub trait Log {
    fn info(&self, message: String);
}

#[derive(Fail, Debug)]
#[fail(display = "{} is unimplemented on this target", feature)]
pub struct UnimplementedError {
    feature: String,
}

#[derive(Fail, Debug)]
pub enum CoreError {
    Unavailable,
    Unimplemented(#[fail(cause)] UnimplementedError),
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        use CoreError::{Unavailable, Unimplemented};
        write!(
            formatter,
            "{}",
            match self {
                Unavailable => "this feature is unavailable or unregistered".to_owned(),
                Unimplemented(feature) => format!("{}", feature),
            }
        )
    }
}

struct Logger;

impl Log for Logger {
    fn info(&self, _message: String) {
        #[cfg(all(target_arch = "wasm32", feature = "core"))]
        web_sys::console::log_1(&_message.into());
        #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
        unimplemented!();
        #[cfg(not(target_arch = "wasm32"))]
        println!("{}", _message);
    }
}

mod private {
    use super::{Executor, Log};
    use crate::reflection::Reflected;

    pub trait Sealed {}

    impl<T: Reflected + ?Sized> Sealed for T {}
    impl Sealed for dyn Executor {}
    impl Sealed for dyn Log {}
}

/// A type retrievable from `core`.
///
/// This trait is implemented for any reflected (i.e. `object`-ified) trait object as well as the special-cased executor trait objects
/// that do not also implement `Kind`. It is bounded on a sealed trait and therefore cannot be implemented by third-party crates.
pub trait CoreValue: private::Sealed {}

impl<T: ?Sized> CoreValue for T where T: private::Sealed {}

pub fn core<T: Any + ?Sized + CoreValue>() -> Result<Box<T>, CoreError> {
    let ty = TypeId::of::<T>();
    if ty == TypeId::of::<dyn Executor>() {
        return executor::new_executor()
            .map(|executor| *Box::<dyn Any>::downcast(Box::new(executor) as Box<dyn Any>).unwrap())
            .map_err(CoreError::Unimplemented);
    }
    if ty == TypeId::of::<dyn Log>() {
        #[cfg(all(target_arch = "wasm32", not(feature = "core")))]
        return Err(CoreError::Unimplemented(UnimplementedError {
            feature: "logging".to_string(),
        }));
        #[cfg(not(all(target_arch = "wasm32", not(feature = "core"))))]
        return Ok(*Box::<dyn Any>::downcast(
            Box::new(Box::new(Logger) as Box<dyn Log>) as Box<dyn Any>
        )
        .unwrap());
    }
    Err(CoreError::Unavailable)
}
