use vessels::{kind::Future, object};

#[object]
pub trait Test {
    fn test(&self) -> Future<()>;
}
