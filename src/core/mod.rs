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

pub fn core<T: Any>() -> Result<T, CoreError> {
    let ty = TypeId::of::<T>();
    if ty == TypeId::of::<Executor>() {
        return executor::new_executor()
            .map(|executor| *Box::<dyn Any>::downcast(Box::new(executor) as Box<dyn Any>).unwrap())
            .map_err(CoreError::Unimplemented);
    }
    Err(CoreError::Unavailable)
}
