use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{
    Color, ContextGraphics, ContextualGraphics, Frame, Graphics, Transform,
};
use vitruvia::path::{Path, Primitive};

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let path: Path = Primitive::square(50.)
        .fill(Color::black().into())
        .finalize()
        .with_origin((25., 25.));
    let mut object = root.add(path, (0., 0.));
    let translate: Transform = (75., 75.).into();
    object.apply_transform(translate.with_rotation(1.));
    let ctx = gfx.start(root);
    ctx.run();
}
