use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Color, Content, Transform};
use vitruvia::path::{GradientStop, LinearGradient, Path, Primitive, StrokeBuilder, Texture};
use vitruvia::text::Text;

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let gradient = Texture::LinearGradient(LinearGradient {
        stops: vec![
            GradientStop::new(0.0, Color::black()).unwrap(),
            GradientStop::new(1.0, Color::white()).unwrap(),
        ],
        start: (0.0, 0.0).into(),
        end: (100.0, 100.0).into(),
    });
    let stroke_gradient = StrokeBuilder::new(gradient, 10.0).finalize();
    let path: Path = Primitive::square(100.).stroke(stroke_gradient).finalize();
    let mut object = root.add(path.into());
    let translate: Transform = (100., 100.).into();
    object.apply_transform(translate);
    root.add(
        Content::from(
            Text::new("Haha yes text")
                .with_letter_spacing(-0.5)
                .with_color(Color::black()),
        )
        .with_transform((200., 200.).into()),
    );
    let ctx = gfx.start(root);
    ctx.run(Box::new(|_| {}));
}
