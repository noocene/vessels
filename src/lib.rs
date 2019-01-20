#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[macro_use]
extern crate stdweb;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[macro_use]
extern crate serde_derive;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
#[macro_use]
extern crate stdweb_derive;

pub(crate) mod render;
mod targets;

pub use crate::render::{Frame, Point, Rect, Renderer, Size};

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub fn initialize() -> impl Renderer {
    targets::web::WebGL2::new()
}
