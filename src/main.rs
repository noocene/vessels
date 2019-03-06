use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, Frame2D, Graphics2D, Object2D, StaticObject2D,  RGBA8,
};
use vitruvia::graphics::path::{
    StrokeBuilder, Shadow2D,GeometryPrimitive,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut rrect = StaticObject2D::from_entity(
        GeometryPrimitive::rounded_square(200., 10.)
            .fill(RGBA8::black().with_alpha(255).into())
            .finalize(),
    );
    if let Object2D::Static(object) = &mut rrect {
        object.orientation.translate(100., 100.);
    };
    root.add(rrect);
    gfx.run(root);
}
