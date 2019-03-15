use vitruvia::graphics;
use vitruvia::graphics::path::Primitive;
use vitruvia::graphics::text::Text;
use vitruvia::graphics::{
    ContextualGraphics2D, Frame2D, Graphics2D, ImageRepresentation, Rasterizer, StaticObject2D,
    RGBA8,
};
use vitruvia::input::keyboard;
use vitruvia::input::{Context, Source};

#[macro_use]
extern crate stdweb;

fn main() {
    let ctx = graphics::new();
    let mut root = ctx.frame();

    let text = ctx.rasterize(Text::new("testing the thing that allows text rendering hello there テスト ensure CJK works word wrap functions over an arbitrary number of lines").with_color(RGBA8::black().with_alpha(190)).with_max_width(170).wrap());

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

    let ctx = ctx.run(root);

    let keyboard = ctx.keyboard();

    keyboard.bind(move |event: keyboard::Event| {
        if let keyboard::Event::Down(key) = event {
            console!(log, format!("{:?}", key))
        }
    });
}
