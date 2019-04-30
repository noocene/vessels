use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Color, Transform};
use vitruvia::path::{Path, Primitive};
use vitruvia::text::Text;

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let path: Path = Primitive::square(100.)
        .fill(Color::black().into())
        .finalize();
    let mut object = root.add(path.into(), (0., 0.).into());
    let translate: Transform = (100., 100.).into();
    object.apply_transform(translate);
    root.add(
        Text::new("Haha yes text").with_color(Color::black()).into(),
        (200., 200.).into(),
    );
    let ctx = gfx.start(root);
    ctx.run(Box::new(|_| {}));
}
