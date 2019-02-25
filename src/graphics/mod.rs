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

pub trait Object<'a, T>
where
    T: Representation,
{
}

pub trait Frame<'a, T>: Object<'a, T>
where
    T: Representation,
{
    fn add(&self, object: &'a Object<T>);
    fn resize(&self, size: Size);
}

pub trait GraphicsEmpty<'a> {}

pub trait Graphics<'a, R>
where
    R: Representation,
{
    fn run(&self, root: &'a Frame<R>);
    fn frame(&self) -> Box<Frame<R>>;
}

pub trait Graphics2D<'a>:
    Graphics<'a, <Self as Graphics2D<'a>>::R> + TryFrom<Self, Error = ()>
{
    type R: Euclidean2D;
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

mod targets;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn initialize<'a>() -> Box<GraphicsEmpty<'a>> {
    targets::web::canvas::initialize()
}
