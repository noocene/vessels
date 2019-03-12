use vitruvia::graphics;
use vitruvia::graphics::path::Primitive;
use vitruvia::graphics::text::Text;
use vitruvia::graphics::{
    Frame2D, Graphics2D, ImageRepresentation, Rasterizer, StaticObject2D, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();

    let text = gfx.rasterize(Text::new("testing the thing that allows text rendering hello there テスト ensure CJK works word wrap functions over an arbitrary number of lines").with_color(RGBA8::black().with_alpha(190)).with_max_width(170).wrap());

    root.add(
        StaticObject2D::from_entity(
            Primitive::rounded_rectangle((200., text.get_size().y + 30.), 5.)
                .fill(RGBA8::black().with_alpha(10).into())
                .finalize(),
        )
        .with_transform(|transform| {
            transform.translate((50., 50.));
        }),
    );

    root.add(StaticObject2D::from(text).with_transform(|transform| {
        transform.translate((65., 65.));
    }));
    gfx.run(root);
}
