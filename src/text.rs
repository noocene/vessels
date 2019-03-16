use crate::graphics_2d::*;

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

impl<'a> Text<'a> {
    pub fn new(content: &str) -> Text<'_> {
        Text::default().with_content(content)
    }
    fn with_content(mut self, content: &'a str) -> Self {
        self.content = content;
        self
    }
    pub fn with_color(mut self, color: RGBA8) -> Self {
        self.color = color;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn with_size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }
    pub fn with_line_height(mut self, line_height: u16) -> Self {
        self.line_height = line_height;
        self
    }
    pub fn with_max_width(mut self, max_width: u32) -> Self {
        self.max_width = Some(max_width);
        self
    }
    pub fn wrap(mut self) -> Self {
        self.wrap = Wrap::Normal;
        self
    }
    pub fn center(mut self) -> Self {
        self.align = Align::Center;
        self
    }
    pub fn justify_end(mut self) -> Self {
        self.align = Align::End;
        self
    }
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }
}

impl<'a> Default for Text<'a> {
    fn default() -> Text<'a> {
        Text {
            font: Font::SystemFont,
            content: "",
            size: 15,
            color: RGBA8::black(),
            italic: false,
            max_width: None,
            align: Align::Start,
            line_height: 26,
            wrap: Wrap::None,
            weight: Weight::Normal,
        }
    }
}

impl<'a> Into<Rasterizable<'a>> for Text<'a> {
    fn into(self) -> Rasterizable<'a> {
        Rasterizable::Text(self)
    }
}
