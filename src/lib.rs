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

pub use crate::render::{Frame, Geometry, Geometry2D, Geometry3D, Point, Rect, Renderer, Size};

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
pub fn initialize() -> impl Renderer {
    targets::web::WebGL2::new()
}
