use std::convert::TryFrom;
use std::error::Error;

pub trait Geometry {}

pub trait Geometry2D: Geometry {}

pub trait Geometry3D: Geometry {}

pub trait Material {}

pub trait Material2D: Material {}

pub trait Material3D: Material {}

pub trait TextureTarget<O>: Material2D + RenderTarget<O>
where
    O: ?Sized + Object<Geometry, Material>,
{
}

pub trait Object<G, M>
where
    G: ?Sized + Geometry,
    M: ?Sized + Material,
{
}

pub trait RenderTarget<O: ?Sized + Object<Geometry, Material>> {}

pub trait Frame<O>: Object<Geometry2D, TextureTarget<O>>
where
    O: ?Sized + Object<Geometry, Material>,
{
    fn add(&self, object: O);
}

pub trait Renderer {
    fn new() -> Box<Self>
    where
        Self: Sized;
    fn run(&self, root: Box<Frame<Object<Geometry, Material>>>);
}

pub trait RendererSupports2D<'a>: Renderer + TryFrom<&'a Renderer, Error = ()> {}
