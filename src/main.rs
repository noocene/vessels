use vitruvia::path::Primitive;
use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Transform, Color};

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let mut squares = vec![];
    for i in 0..100_000 {
        squares.push({
            let mut object = root.add(Primitive::square(1.).fill(Color::rgba(0, 255, 0, 255).into()).finalize().into());
            object.apply_transform(Transform::default().with_position(i as f64));
            object
        });
    }
    root.add(Primitive::square(100.).fill(Color::rgba(0, 0, 255, 255).into()).finalize().into())
        .apply_transform(Transform::default().with_position((500., 0.)));
    let mut ctx = gfx.start(root);
    ctx.run();
}
