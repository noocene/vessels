use vitruvia::graphics;
use vitruvia::graphics::path::{Path, Primitive, StrokeBuilder, Texture};
use vitruvia::graphics::text::{Font, FontWeight, Text, WordWrap};
use vitruvia::graphics::{
    DynamicObject2D, Frame2D, Graphics2D, ImageRepresentation, Object2D, Rasterizer,
    StaticObject2D, Transform2D, Vec2D, RGBA8,
};

use std::borrow::Cow;

use std::f64::consts::FRAC_PI_4;

pub struct TextObject<T>
where
    T: ImageRepresentation,
{
    texture: T,
}

impl<T> TextObject<T>
where
    T: ImageRepresentation,
{
    pub fn new(texture: T) -> Self {
        TextObject { texture }
    }
    pub fn get_size(&self) -> Vec2D {
        self.texture.get_size()
    }
}

impl<T> DynamicObject2D<T> for TextObject<T>
where
    T: ImageRepresentation,
{
    fn orientation(&self) -> Transform2D {
        Transform2D::default().with_position((65., 65.))
    }
    fn render(&self) -> Cow<[Path<T>]> {
        Cow::from(vec![Primitive::rectangle(self.texture.get_size())
            .fill(Texture::Image(Box::new(self.texture.clone())).into())
            .finalize()])
    }
}

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();

    let text = TextObject::new(gfx.rasterize(Text {
        weight: FontWeight::Normal,
        content: "testing the thing that allows text rendering hello there テスト ensure CJK works word wrap functions over an arbitrary number of lines",
        font: Font::SystemFont,
        italic: false,
        max_width: Some(170),
        color: RGBA8::black().with_alpha(180),
        size: 15,
        word_wrap: WordWrap::Normal,
        line_height: 26,
    }));

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

    root.add(Object2D::Dynamic(Box::new(text)));
    gfx.run(root);
}
