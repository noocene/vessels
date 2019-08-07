use vitruvia::{path::{Primitive, Shadow}, graphics_2d::{self, Transform, Color}};

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    root.add(Primitive::square(100.).shadow(Shadow::new(Color::black().with_alpha(75)).offset((-10., -10.)).spread(10.).blur(10.)).shadow(Shadow::new(Color::black().with_alpha(150)).offset((10., 10.)).spread(10.).blur(10.)).fill(Color::rgba(0, 0, 255, 255).into()).finalize().into())
        .apply_transform(Transform::default().with_position((50., 50.)));
    gfx.start(root).run();
}
