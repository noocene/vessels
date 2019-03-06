use vitruvia::graphics;
use vitruvia::graphics::path::Primitive;
use vitruvia::graphics::{Frame2D, Graphics2D, Object2D, StaticObject2D, RGBA8};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut rrect = StaticObject2D::from_entity(
        Primitive::circle(200.)
            .fill(RGBA8::black().with_alpha(255).into())
            .finalize(),
    );
    if let Object2D::Static(object) = &mut rrect {
        object.orientation.translate(50., 50.);
    };
    root.add(rrect);
    gfx.run(root);
}
