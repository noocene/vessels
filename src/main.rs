use vitruvia::graphics;
use vitruvia::graphics::path::{Path, Primitive, StrokeBuilder, Texture};
use vitruvia::graphics::text::{Font, FontWeight, Text};
use vitruvia::graphics::{
    DynamicObject2D, Frame2D, Graphics2D, ImageRepresentation, Object2D, Rasterizer,
    StaticObject2D, Transform2D, RGBA8,
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
}

impl<T> DynamicObject2D<T> for TextObject<T>
where
    T: ImageRepresentation,
{
    fn orientation(&self) -> Transform2D {
        Transform2D::default().with_position((67.5, 25.))
    }
    fn render(&self) -> Cow<[Path<T>]> {
        Cow::from(vec![Primitive::square(100000.)
            .fill(Texture::Image(Box::new(self.texture.clone())).into())
            .finalize()])
    }
}

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();

    root.add(
        StaticObject2D::from_entity(
            Primitive::rounded_rectangle((100., 40.), 5.)
                .fill(RGBA8::black().with_alpha(20).into())
                .finalize(),
        )
        .with_transform(|transform| {
            transform.translate((50., 50.));
        }),
    );

    let text = TextObject::new(gfx.rasterize(Text {
        weight: FontWeight::Normal,
        content: "testing",
        font: Font::SystemFont,
        italic: true,
        color: RGBA8::black().with_alpha(160),
        size: 15,
    }));

    root.add(Object2D::Dynamic(Box::new(text)));
    gfx.run(root);
}
