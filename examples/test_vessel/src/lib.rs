use vessels::{kind::Infallible, object};

#[object]
pub trait Test {
    fn test(&self, message: String) -> Infallible<String>;
}
