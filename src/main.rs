use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Color, Transform};
use vitruvia::path::{Path, Primitive};

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let path: Path = Primitive::square(100.)
        .fill(Color::black().into())
        .finalize();
    let mut object = root.add(path.into(), (0., 0.).into());
    let translate: Transform = (100., 100.).into();
    object.apply_transform(translate);
    let ctx = gfx.start(root);
    ctx.run(Box::new(|_| {}));
}
