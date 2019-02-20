pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}
