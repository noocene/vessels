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

use std::convert::TryFrom;

pub(crate) mod render;
mod targets;

pub use crate::render::{
    Frame, Geometry, Material, Object, RenderTarget, Renderer, RendererSupports2D,
};

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn initialize() -> impl Renderer {
    targets::web::WebGL2::new()
}

pub struct E {}

impl Renderer for E {
    fn new() -> Box<Self> {
        Box::new(E {})
    }
    fn run(&self, root: Box<Frame<Object<Geometry, Material>>>) {}
}

impl<'a> RendererSupports2D<'a> for E {}

impl<'a> TryFrom<&'a Renderer> for E {
    type Error = ();
    fn try_from(input: &'a Renderer) -> Result<E, Self::Error> {
        Ok(E {})
    }
}

pub fn initialize() -> Box<dyn Renderer> {
    E::new()
}
