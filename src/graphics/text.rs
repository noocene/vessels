use crate::graphics::path::*;
use crate::graphics::*;

pub enum Font {
    SystemFont,
}

pub enum Weight {
    Hairline,
    Light,
    Thin,
    Normal,
    Bold,
    Heavy,
}

pub enum Wrap {
    None,
    Normal,
}

pub enum Align {
    Center,
    Start,
    End,
}

pub struct Text<'a> {
    pub font: Font,
    pub content: &'a str,
    pub size: u16,
    pub color: RGBA8,
    pub italic: bool,
    pub max_width: Option<u32>,
    pub align: Align,
    pub line_height: u16,
    pub wrap: Wrap,
    pub weight: Weight,
}

impl<'a> Into<Rasterizable<'a>> for Text<'a> {
    fn into(self) -> Rasterizable<'a> {
        Rasterizable::Text(self)
    }
}
