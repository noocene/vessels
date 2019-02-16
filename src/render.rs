pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

pub trait Geometry {}

pub trait Geometry2D: Geometry {}

pub trait Geometry3D: Geometry {}

pub trait Material {}

pub trait Material2D: Material {}

pub trait Material3D: Material {}

pub trait TextureTarget<O>: Material2D + RenderTarget<O>
where
    O: ?Sized + Object<dyn Geometry, dyn Material>,
{
}

pub trait Object<G, M>
where
    G: ?Sized + Geometry,
    M: ?Sized + Material,
{
}

pub trait RenderTarget<O: ?Sized + Object<dyn Geometry, dyn Material>> {}

pub trait Frame<'a, O>: Object<Geometry2D, TextureTarget<O>>
where
    O: ?Sized + Object<dyn Geometry, dyn Material>,
{
    fn add(&self, object: &'a O);
}

pub trait GraphicsEmpty: Graphics + TryInto<Box<dyn Graphics2D>, Error = ()> {
}

pub trait Graphics {
    fn new() -> Self where Self: Sized;
    fn run(&self, root: Box<dyn Frame<Object<dyn Geometry, dyn Material>>>);
}

pub trait Graphics2D: Graphics {
    fn frame(&mut self) -> Frame2D;
}

pub type TextureTarget2D = dyn TextureTarget<Object2D>;

pub type Object2D = dyn Object<dyn Geometry2D, dyn Material2D>;

pub type Object3D = dyn Object<dyn Geometry3D, dyn Material3D>;

pub type Frame2D<'a> = Box<dyn Frame<'a, Object2D>>;
