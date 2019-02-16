#![feature(try_from)]

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
#[macro_use]
extern crate stdweb;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
#[macro_use]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
#[macro_use]
extern crate stdweb_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
extern crate weak_table;

pub(crate) mod render;
mod targets;

pub use crate::render::{
    Frame, Geometry, Material, Object, RenderTarget, Graphics, GraphicsEmpty, TryInto, Geometry2D, Material2D, TextureTarget, TextureTarget2D, Object2D, Frame2D
};

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn initialize() -> impl Renderer {
    targets::web::WebGL2::new()
}

pub struct M {}

impl Material for M {}

impl Material2D for M {}

pub struct S {}

impl Geometry for S {}

impl Geometry2D for S {}

pub struct G {}

impl Graphics for G {
    fn new() -> Self {
        G {}
    }
    fn run(&self, _root: Box<Frame<Object<Geometry, Material>>>) {}
}

impl GraphicsEmpty for G {
}

impl render::Graphics2D for G {
    fn frame(&mut self) -> Frame2D {
        Box::new(F {})
    }
}

pub struct F {
}

impl Object<dyn Geometry2D, TextureTarget2D> for F {
}

impl<'a> Frame<'a, Object2D> for F {
    fn add(&self, _object: &'a Object2D) {
    }
}

impl TryInto<Box<render::Graphics2D>> for G {
    type Error = ();
    fn try_into(self) -> Result<Box<dyn render::Graphics2D>, Self::Error> {
        Ok(Box::new(self))
    }
}

pub fn initialize() -> impl GraphicsEmpty {
    G::new()
}

pub type Graphics2D = Box<render::Graphics2D>;
