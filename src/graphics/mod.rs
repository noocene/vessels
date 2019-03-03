use crate::util::*;

pub struct PathGeometry2DBuilder {}

pub struct PathGeometry2D {}

pub trait Object2D {}

pub trait Frame2D: Object2D {
    fn add(&self, object: Box<Object2D>);
    fn resize(&self, size: Size);
}

pub trait Graphics2D {
    type Frame: Frame2D;
    fn run(&self, root: Self::Frame);
    fn frame(&self) -> Self::Frame;
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

mod targets;

pub fn new() -> impl Graphics2D {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
    targets::web::new()
}
