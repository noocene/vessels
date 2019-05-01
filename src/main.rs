use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Color, Content, Transform};
use vitruvia::path::{Shadow, Path, Primitive};
use vitruvia::text::Text;

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let path: Path = Primitive::square(100.).fill(Color::black().into()).shadow(Shadow::new(Color::black().with_alpha(200)).blur(10.)).finalize();
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
