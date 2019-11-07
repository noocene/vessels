use failure::Fail;
use std::{
    any::{Any, TypeId},
    fmt::{self, Display, Formatter},
};

pub mod executor;
pub use executor::Executor;

#[derive(Fail, Debug)]
#[fail(display = "{} is unimplemented on this target", feature)]
pub struct UnimplementedError {
    feature: String,
}

#[derive(Fail, Debug)]
pub enum CoreError {
    Unavailable,
    Unimplemented(UnimplementedError),
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

mod private {
    use super::Executor;
    use crate::reflection::Reflected;

    pub trait Sealed {}

    impl<T: Reflected + ?Sized> Sealed for T {}
    impl Sealed for dyn Executor {}
}

pub trait CoreValue: private::Sealed {}

impl<T: ?Sized> CoreValue for T where T: private::Sealed {}

pub fn core<T: Any + ?Sized + CoreValue>() -> Result<Box<T>, CoreError> {
    let ty = TypeId::of::<T>();
    if ty == TypeId::of::<dyn Executor>() {
        return executor::new_executor()
            .map(|executor| *Box::<dyn Any>::downcast(Box::new(executor) as Box<dyn Any>).unwrap())
            .map_err(CoreError::Unimplemented);
    }
    Err(CoreError::Unavailable)
}
