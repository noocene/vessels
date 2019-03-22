use crate::graphics_2d::{Transform, RGBA8};

/// A font face.
#[derive(Clone, Copy, Debug)]
pub enum Font {
    /// The default system font face used for native UI elements.
    SystemFont,
}

/// Specifies the weight of a font.
#[derive(Clone, Copy, Debug)]
pub enum Weight {
    /// Hairline weight.
    Hairline,
    /// Light weight.
    Light,
    /// Thin weight.
    Thin,
    /// Normal font weight.
    Normal,
    /// Standard bold font weight.
    Bold,
    /// A heavy/black font weight.
    Heavy,
}

/// Specifies the type of text wrap used.
#[derive(Clone, Copy, Debug)]
pub enum Wrap {
    /// No wrap.
    None,
    /// Standard word-level text wrap.
    Normal,
}

/// Specifices the alignment of text.
#[derive(Clone, Copy, Debug)]
pub enum Align {
    /// Center-justification.
    Center,
    /// Left-justification.
    Start,
    /// Right-justification.
    End,
}

/// Provides an abstraction for representing text content.
#[derive(Clone, Debug)]
pub struct Text {
    /// The font face used.
    pub font: Font,
    /// The actual text content to render.
    pub content: String,
    /// The font size in pixels.
    pub size: u16,
    /// The color of the rendered text.
    pub color: RGBA8,
    /// Whether the text is styled as oblique/italic.
    pub italic: bool,
    /// The maximum width or wrap width of the text.
    pub max_width: Option<u32>,
    /// The justification or alignment style of the text.
    pub align: Align,
    /// The line height in pixels.
    pub line_height: u16,
    /// The type of text wrap used.
    pub wrap: Wrap,
    /// The font weight used.
    pub weight: Weight,
}

impl Text {
    /// Creates a new text abstraction with the given content.
    pub fn new(content: &str) -> Text {
        Text::default().with_content(content)
    }
    fn with_content(mut self, content: &'_ str) -> Self {
        self.content = content.to_owned();
        self
    }
    /// Sets the color of the text.
    pub fn with_color(mut self, color: RGBA8) -> Self {
        self.color = color;
        self
    }
    /// Makes the text italic.
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    /// Sets the font size of the text.
    pub fn with_size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }
    /// Sets the line height of the text.
    pub fn with_line_height(mut self, line_height: u16) -> Self {
        self.line_height = line_height;
        self
    }
    /// Sets the max width of the text.
    pub fn with_max_width(mut self, max_width: u32) -> Self {
        self.max_width = Some(max_width);
        self
    }
    /// Enables text wrapping.
    pub fn wrap(mut self) -> Self {
        self.wrap = Wrap::Normal;
        self
    }
    /// Sets the text alignment to be centered.
    pub fn center(mut self) -> Self {
        self.align = Align::Center;
        self
    }
    /// Sets the text alignment to be right-justified.
    pub fn justify_end(mut self) -> Self {
        self.align = Align::End;
        self
    }
    /// Sets the font weight.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }
}

impl Default for Text {
    fn default() -> Text {
        Text {
            orientation: Transform::default(),
            font: Font::SystemFont,
            content: "".to_owned(),
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
