use failure::Fail;
use std::{
    any::{Any, TypeId},
    fmt::{self, Display, Formatter},
};

pub type MethodIndex = u8;

pub struct MethodTypes {
    pub arguments: Vec<TypeId>,
    pub output: TypeId,
}

#[derive(Debug, Fail)]
pub enum CallError {
    Type(u8),
    ArgumentCount(ArgumentCountError),
    OutOfRange(OutOfRangeError),
}

impl Display for CallError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use CallError::{ArgumentCount, OutOfRange, Type};

        write!(
            f,
            "{}",
            match self {
                Type(position) => format!("invalid type for argument {}", position),
                OutOfRange(error) => format!("{}", error),
                ArgumentCount(error) => format!("{}", error),
            }
        )
    }
}

#[derive(Debug, Fail)]
#[fail(display = "method {} out of range", index)]
pub struct OutOfRangeError {
    pub index: MethodIndex,
}

#[derive(Debug, Fail)]
#[fail(display = "got {} arguments, expected {}", got, expected)]
pub struct ArgumentCountError {
    pub expected: usize,
    pub got: usize,
}

#[derive(Debug, Fail)]
#[fail(display = "no method with name {}", name)]
pub struct NameError {
    pub name: String,
}

pub trait Trait {
    fn call(
        &mut self,
        index: MethodIndex,
        args: Vec<Box<dyn Any + Send>>,
    ) -> Result<Box<dyn Any + Send>, CallError>;
    fn by_name(&self, name: &'_ str) -> Result<MethodIndex, NameError>;
    fn count(&self) -> MethodIndex;
    fn name_of(&self, index: MethodIndex) -> Result<String, OutOfRangeError>;
    fn types(&self, index: MethodIndex) -> Result<MethodTypes, OutOfRangeError>;
}
