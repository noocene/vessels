use crate::graphics::path::*;
use crate::graphics::*;

pub enum Font {
    SystemFont,
}

pub enum FontWeight {
    Hairline,
    Light,
    Thin,
    Normal,
    Bold,
    Heavy,
}

pub enum WordWrap {
    None,
    Normal,
}

pub struct Text<'a> {
    pub font: Font,
    pub content: &'a str,
    pub size: u16,
    pub color: RGBA8,
    pub italic: bool,
    pub max_width: Option<u32>,
    pub line_height: u16,
    pub word_wrap: WordWrap,
    pub weight: FontWeight,
}

impl<'a> Into<Rasterizable<'a>> for Text<'a> {
    fn into(self) -> Rasterizable<'a> {
        Rasterizable::Text(self)
    }
}
