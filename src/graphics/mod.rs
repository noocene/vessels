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

pub trait GraphicsEmpty {}

pub trait Graphics<R>
where
    R: Representation,
{
    fn run(&self, root: Box<Frame<R>>);
    fn frame(&self) -> Box<Frame<R>>;
}

pub trait Graphics2D: Graphics<<Self as Graphics2D>::R> + TryFrom<Self, Error = ()> {
    type R: Euclidean2D;
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

mod targets;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn initialize() -> Box<GraphicsEmpty> {
    targets::web::canvas::initialize()
}
