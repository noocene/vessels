use crate::util::*;

pub trait Representation: Sized {}

pub trait Euclidean2D: Representation {}

pub trait Geometry<Representation> {}

pub trait Geometry2D<T>: Geometry<T>
where
    T: Euclidean2D,
{
}

pub trait Material<Representation> {}

pub trait Material2D<T>: Material<T>
where
    T: Euclidean2D,
{
}

pub trait Object<T>
where
    T: Representation,
{
}

pub trait Frame<T>: Object<T>
where
    T: Representation,
{
    fn add(&self, object: Box<Object<T>>);
    fn resize(&self, size: Size);
}

pub struct AbstractGraphics {}

pub trait Graphics {
    type Representation: Representation;
    fn run(&self, root: Box<Frame<Self::Representation>>);
    fn frame(&self) -> Box<Frame<Self::Representation>>;
}

pub trait Graphics2D: Graphics
where
    Self::Representation: Euclidean2D,
    Box<Self>: TryFrom<AbstractGraphics, Error = ()>,
{
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

mod targets;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub type AbstractGraphics2D = Box<targets::web::canvas::Canvas>;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn new() -> AbstractGraphics {
    AbstractGraphics {}
}
