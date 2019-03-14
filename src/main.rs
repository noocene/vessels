use vitruvia::graphics;
use vitruvia::graphics::path::Primitive;
use vitruvia::graphics::text::Text;
use vitruvia::graphics::{
    ContextualGraphics2D, Frame2D, Graphics2D, ImageRepresentation, Rasterizer, StaticObject2D,
    RGBA8,
};
use vitruvia::input::mouse;
use vitruvia::input::{Context, Mouse, Source};

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

    let mouse = ctx.mouse();

    mouse.bind(move |event: mouse::Event| {
        let parse = |button: mouse::Button| match button {
            mouse::Button::Left => "left".to_owned(),
            mouse::Button::Right => "right".to_owned(),
            mouse::Button::Middle => "middle".to_owned(),
            mouse::Button::Auxiliary(index) => format!("auxiliary #{}", index),
        };
        console!(
            log,
            format!(
                "at ({}, {}): {}",
                event.position.x,
                event.position.y,
                match event.action {
                    mouse::Action::Down(button) => format!("{} down", parse(button)),
                    mouse::Action::Up(button) => format!("{} up", parse(button)),
                    mouse::Action::Move(delta) => format!("move ({}, {})", delta.x, delta.y),
                }
            )
        )
    });
}
