use vitruvia::graphics_2d;
use vitruvia::graphics_2d::{Color, Content, Transform};
use vitruvia::path::{Shadow, Path, Primitive};
use vitruvia::text::Text;

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    let path: Path = Primitive::rounded_square(200., 10.).fill(Color::black().with_alpha(30).into())/*.shadow(Shadow::new(Color::black().with_alpha(200)).blur(10.))*/.finalize();
    let mut object = root.add(path.into());
    let translate: Transform = (200., 200.).into();
    object.apply_transform(translate);
    root.add(
        Content::from(
            Text::new("Correct wrap now text wrap works lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
                .with_color(Color::black())
                .wrap()
                .with_max_width(200.),
        )
        .with_transform((200., 200.).into()),
    );
    let ctx = gfx.start(root);
    ctx.run(Box::new(|_| {}));
}
