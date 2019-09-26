use crate::{
    graphics::{
        path::{Path, Primitive},
        text::Text,
        ImageRepresentation, Rect, Transform2, Vector2,
    },
    interaction::Context,
    targets,
};

use std::any::Any;

/// Represents content optimized and cached for rendering.
pub trait Object: Sync + Send {
    /// Composes a transformation with the existing transformation of the [Object].
    fn apply_transform(&mut self, transform: Transform2);
    /// Gets the current trasnformation of the [Object].
    fn get_transform(&self) -> Transform2;
    /// Sets the current transfomration of the [Object].
    fn set_transform(&mut self, transform: Transform2);
    /// Gets the current z-depth of the [Object].
    fn get_depth(&self) -> u32;
    /// Sets the current z-depth of the [Object].
    fn set_depth(&mut self, depth: u32);
    /// Replaces the contents of the [Object] with new Rasterizable content. This may be costly.
    fn update(&mut self, content: Rasterizable);
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn Object>;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.box_clone()
    }
}

/// An isolated rendering context.
pub trait Frame: Sync + Send {
    /// Adds content to the [Frame].
    fn add(&mut self, content: Content) -> Box<dyn Object>;
    /// Resizes the [Frame]. This does not resize the viewport.
    fn resize(&self, size: Vector2);
    /// Sets the viewport.
    fn set_viewport(&self, viewport: Rect);
    /// Returns the size of the [Frame].
    fn get_size(&self) -> Vector2;
    /// Returns an image that is a still rasterization of any rendered content.
    fn to_image(&self) -> Box<dyn ImageRepresentation>;
    /// Returns the measured dimensions of some provided content.
    fn measure(&self, interaction: Rasterizable) -> Vector2;
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn Frame>;
    #[doc(hidden)]
    fn show(&self);
    #[doc(hidden)]
    fn draw(&self);
    #[doc(hidden)]
    fn set_pixel_ratio(&self, ratio: f64);
    #[doc(hidden)]
    fn as_any(&self) -> Box<dyn Any>;
}

impl Clone for Box<dyn Frame> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Renderable content.
#[derive(Debug, Clone)]
pub struct Content {
    pub(crate) content: Rasterizable,
    pub(crate) depth: u32,
    pub(crate) transform: Transform2,
}

impl Content {
    /// Sets the orientation of the content.
    pub fn with_transform(mut self, transform: Transform2) -> Self {
        self.transform = transform;
        self
    }
    /// Applies a transformation to the content.
    pub fn apply_transform(&mut self, transform: Transform2) {
        self.transform = transform;
    }
    /// Sets the z-depth of the content.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
}

impl From<Path> for Content {
    fn from(interaction: Path) -> Content {
        Content {
            content: interaction.into(),
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Text> for Content {
    fn from(interaction: Text) -> Content {
        Content {
            content: interaction.into(),
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Rasterizable> for Content {
    fn from(interaction: Rasterizable) -> Content {
        Content {
            content: interaction,
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Content> for Rasterizable {
    fn from(interaction: Content) -> Rasterizable {
        interaction.content
    }
}

/// A type that can be rasterized.
#[derive(Debug, Clone)]
pub enum Rasterizable {
    /// Some [Text].
    Text(Box<Text>),
    /// Some [Path].
    Path(Box<Path>),
}

impl From<Path> for Rasterizable {
    fn from(interaction: Path) -> Rasterizable {
        Rasterizable::Path(Box::new(interaction))
    }
}

impl From<Text> for Rasterizable {
    fn from(interaction: Text) -> Rasterizable {
        Rasterizable::Text(Box::new(interaction))
    }
}

impl From<Box<dyn ImageRepresentation>> for Rasterizable {
    fn from(interaction: Box<dyn ImageRepresentation>) -> Rasterizable {
        Rasterizable::Path(Box::new(
            Primitive::rectangle(interaction.get_size())
                .fill(interaction.into())
                .finalize(),
        ))
    }
}

/// Provides an interface for the rasterization of content.
pub trait Rasterizer: Sync + Send {
    /// Returns a rasterization of the interaction.
    fn rasterize(&self, interaction: Rasterizable, vector: Vector2)
        -> Box<dyn ImageRepresentation>;
}

/// Provides 2-dimensional euclidean rendering capabilities.
pub trait Graphics: Rasterizer {
    /// Returns a new [Frame].
    fn frame(&self) -> Box<dyn Frame>;
}

/// An aggregated context with bound graphics.
pub trait ContextGraphics: Graphics + Context + Ticker {}

impl Clone for Box<dyn ActiveContextGraphics> {
    fn clone(&self) -> Box<dyn ActiveContextGraphics> {
        self.box_clone()
    }
}

/// An active [ContextualGraphics] context.
pub trait ActiveContextGraphics: ContextGraphics {
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn ActiveContextGraphics>;
}

/// An inactive [ContextualGraphics] context.
pub trait InactiveContextGraphics: ContextGraphics {
    /// Begins execution of the runloop. Consumes the context and blocks forever where appropriate. Calls the provided callback once upon execution and moves an active context graphics into it.
    fn run_with(self: Box<Self>, cb: Box<dyn FnMut(Box<dyn ActiveContextGraphics>) + 'static>);
    /// Begins execution of the runloop. Consumes the context and blocks forever where appropriate.
    fn run(self: Box<Self>);
}

/// A type that permits the binding of tick handlers.
pub trait Ticker {
    /// Binds a handler to receive ticks.
    fn bind(&mut self, handler: Box<dyn FnMut(f64) + 'static + Send + Sync>);
}

/// A graphics context that can provide interaction and windowing.
pub trait ContextualGraphics: Graphics {
    /// Starts a windowed context using the provided [Frame] as the document root.
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveContextGraphics>;
}

/// Initializes a new graphics context.
pub fn new() -> Box<dyn ContextualGraphics> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::graphics::new();

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    return targets::native::graphics::new();
}
