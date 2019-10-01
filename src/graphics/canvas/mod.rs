use crate::{
    graphics::{
        path::{Path, Primitive},
        text::Text,
        ImageRepresentation, Rect, Transform2, Vector2,
    },
    input::Provider,
    targets,
};

use std::any::Any;

/// Represents content optimized and cached for rendering.
pub trait Object {
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
pub trait Frame {
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
    fn measure(&self, input: Rasterizable) -> Vector2;
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
    fn from(input: Path) -> Content {
        Content {
            content: input.into(),
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Text> for Content {
    fn from(input: Text) -> Content {
        Content {
            content: input.into(),
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Rasterizable> for Content {
    fn from(input: Rasterizable) -> Content {
        Content {
            content: input,
            depth: 0,
            transform: Transform2::default(),
        }
    }
}

impl From<Content> for Rasterizable {
    fn from(input: Content) -> Rasterizable {
        input.content
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
    fn from(input: Path) -> Rasterizable {
        Rasterizable::Path(Box::new(input))
    }
}

impl From<Text> for Rasterizable {
    fn from(input: Text) -> Rasterizable {
        Rasterizable::Text(Box::new(input))
    }
}

impl From<Box<dyn ImageRepresentation>> for Rasterizable {
    fn from(input: Box<dyn ImageRepresentation>) -> Rasterizable {
        Rasterizable::Path(Box::new(
            Primitive::rectangle(input.get_size())
                .fill(input.into())
                .finalize(),
        ))
    }
}

/// Provides an interface for the rasterization of content.
pub trait Rasterizer {
    /// Returns a rasterization of the input.
    fn rasterize(&self, input: Rasterizable, vector: Vector2) -> Box<dyn ImageRepresentation>;
}

/// Provides 2-dimensional euclidean rendering capabilities.
pub trait Canvas: Rasterizer {
    /// Returns a new [Frame].
    fn frame(&self) -> Box<dyn Frame>;
}

/// An aggregated context with bound graphics.
pub trait CanvasContext: Canvas + Provider {}

impl Clone for Box<dyn ActiveCanvas> {
    fn clone(&self) -> Box<dyn ActiveCanvas> {
        self.box_clone()
    }
}

/// An active canvas.
pub trait ActiveCanvas: CanvasContext {
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn ActiveCanvas>;
}

/// An inactive canvas.
pub trait InactiveCanvas: CanvasContext {
    /// Begins execution of the runloop. Consumes the context and blocks forever where appropriate. Calls the provided callback every frame during execution and provides an active context graphics to it.
    fn run_with(self: Box<Self>, cb: Box<dyn FnMut(Box<dyn ActiveCanvas>) + 'static>);
    /// Begins execution of the runloop. Consumes the context and blocks forever where appropriate.
    fn run(self: Box<Self>);
}

/// A graphics context that can provide input and windowing.
pub trait InteractiveCanvas: Canvas {
    /// Starts a windowed context using the provided [Frame] as the document root.
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveCanvas>;
}

/// Initializes a new graphics context.
pub fn new() -> Box<dyn InteractiveCanvas> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    return targets::web::graphics::new();

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    return targets::native::graphics::new();
}
