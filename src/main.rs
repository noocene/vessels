use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, Frame2D, Graphics2D, Object2D, StaticObject2D, StrokeBuilder, VectorEntity2DShadow,
    VectorGeometryPrimitive, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut rrect = StaticObject2D::from_entity(
        VectorGeometryPrimitive::rounded_square(200., 10.)
            .fill(RGBA8::black().with_alpha(255).into())
            .finalize(),
    );
    if let Object2D::Static(object) = &mut rrect {
        object.orientation.translate(100., 100.);
    };
    root.add(rrect);
    gfx.run(root);
}
