use vitruvia::graphics;
use vitruvia::graphics::path::{Primitive, StrokeBuilder};
use vitruvia::graphics::{Frame2D, Graphics2D, Object2D, StaticObject2D, RGBA8};

use std::f64::consts::FRAC_PI_4;

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut rrect = StaticObject2D::from_entity(
        Primitive::continuous_curvature_square((50.), 0.8)
            .stroke(StrokeBuilder::new(RGBA8::black().into(), 1.).finalize())
            .finalize().with_origin((50., 50.)),
    );
    if let Object2D::Static(object) = &mut rrect {
        object.orientation.translate((100., 100.)).rotate(0.);
    };
    let mut rrect2 = StaticObject2D::from_entity(
        Primitive::continuous_curvature_square(50., 0.8)
            .stroke(StrokeBuilder::new(RGBA8::black().into(), 1.).finalize())
            .finalize().with_origin((50., 50.)),
    );
    if let Object2D::Static(object) = &mut rrect2 {
        object.orientation.translate((100., 100.)).rotate(FRAC_PI_4);
    };
    root.add(rrect);
    root.add(rrect2);
    gfx.run(root);
}
