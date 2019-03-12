use crate::graphics::path::*;
use crate::graphics::*;

pub enum Font {
    SystemFont,
}

pub enum FontWeight {
    Light,
    Thin,
    Normal,
    Bold,
    Heavy,
}

pub struct Text<'a> {
    pub font: Font,
    pub content: &'a str,
    pub size: u16,
    pub color: RGBA8,
    pub italic: bool,
    pub weight: FontWeight,
}

impl<'a> Into<Rasterizable<'a>> for Text<'a> {
    fn into(self) -> Rasterizable<'a> {
        Rasterizable::Text(self)
    }
}
