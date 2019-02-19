use crate::graphics::{
    Frame, Frame2D, Geometry, Geometry2D, Graphics, Graphics2D, GraphicsEmpty, Material,
    Material2D, Object, Object2D, TextureTarget2D, TryInto,
};

pub struct M {}

impl Material for M {}

impl Material2D for M {}

pub struct S {}

impl Geometry for S {}

impl Geometry2D for S {}

pub struct G {}

impl Graphics for G {
    fn run(&self, _root: Box<Frame<Object<Geometry, Material>>>) {}
}

impl GraphicsEmpty for G {}

impl Graphics2D for G {
    fn frame(&mut self) -> Frame2D {
        Box::new(F {})
    }
}

pub struct F {}

impl Object<dyn Geometry2D, TextureTarget2D> for F {}

impl<'a> Frame<'a, Object2D> for F {
    fn add(&self, _object: &'a Object2D) {}
}

impl TryInto<Box<Graphics2D>> for G {
    type Error = ();
    fn try_into(self) -> Result<Box<dyn Graphics2D>, Self::Error> {
        Ok(Box::new(self))
    }
}
