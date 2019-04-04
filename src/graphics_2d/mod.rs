use crate::input::Context;
use crate::path::{Path, Primitive, Texture};
use crate::targets;
use crate::text::Text;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use std::borrow::Cow;

use std::any::Any;

/// A conversion to an eight-character hex color string.
pub trait ToHexColor {
    /// Performs the conversion.
    fn to_hex_color(&self) -> Cow<'_, str>;
}

/// A representation type of some target-specific image format.
pub trait ImageRepresentation: Any + Sync + Send {
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn ImageRepresentation>;
    #[doc(hidden)]
    fn as_any(&self) -> Box<dyn Any>;
    /// Returns the 2-d cartesian pixel size of the image.
    fn get_size(&self) -> Vector;
    /// Returns a conversion of the image to [Image<Color, Texture2D>]. This operation may be expensive.
    fn as_texture(&self) -> Image<Color, Texture2D>;
    /// Creates an image in the associated format from an [Image<Color, Texture2D>]. This operation may be expensive.
    fn from_texture(texture: Image<Color, Texture2D>) -> Self
    where
        Self: Sized;
}

impl Clone for Box<dyn ImageRepresentation> {
    fn clone(&self) -> Box<dyn ImageRepresentation> {
        self.box_clone()
    }
}

impl ImageRepresentation for Image<Color, Texture2D> {
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
    fn get_size(&self) -> Vector {
        (f64::from(self.format.width), f64::from(self.format.height)).into()
    }
    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }
    fn as_texture(&self) -> Image<Color, Texture2D> {
        self.clone()
    }
    fn from_texture(texture: Image<Color, Texture2D>) -> Image<Color, Texture2D> {
        texture
    }
}

/// Indicates that a type is a pixel format for image data.
pub trait PixelFormat {}

/// A standard 24-bit-depth RGB color with an 8-bit alpha channel.
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    /// Red channel data.
    pub r: u8,
    /// Green channel data.
    pub g: u8,
    /// Blue channel data.
    pub b: u8,
    /// Alpha channel data.
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from(input: (u8, u8, u8)) -> Color {
        Color {
            r: input.0,
            g: input.1,
            b: input.2,
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(input: (u8, u8, u8, u8)) -> Color {
        Color {
            r: input.0,
            g: input.1,
            b: input.2,
            a: input.3,
        }
    }
}

impl Color {
    /// Returns a CSS-compatible rgba color string in form `rgba(r, g, b, a)` where `r`, `g`, and `b`
    /// are integers between 0 and 255 and `a` is the alpha channel represented as a floating point value between
    /// 0 and 1.
    pub fn to_rgba_color(&self) -> Cow<'_, str> {
        Cow::from(format!(
            "rgba({},{},{},{})",
            self.r,
            self.g,
            self.b,
            f64::from(self.a) / 255.
        ))
    }
    /// Sets the alpha channel byte.
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.a = alpha;
        self
    }
    /// Returns a fully opaque black color.
    pub fn black() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    /// Returns a fully opaque white color.
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

impl ToHexColor for Color {
    fn to_hex_color(&self) -> Cow<'_, str> {
        Cow::from(format!(
            "#{:x?}{:x?}{:x?}{:x?}",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl Into<Texture> for Color {
    fn into(self) -> Texture {
        Texture::Solid(self)
    }
}

impl PixelFormat for Color {}

/// Indicates that a type is an organizational format for image data.
pub trait ImageFormat {}

/// A typical two-dimensional grid image format with square pixels.
#[derive(Clone, Copy, Debug)]
pub struct Texture2D {
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
}

impl ImageFormat for Texture2D {}

/// A concrete image composed of format data and a flat [Vec] of pixels
#[derive(Clone, Debug)]
pub struct Image<T: PixelFormat, U: ImageFormat> {
    /// Pixel data.
    pub pixels: Vec<T>,
    /// Format of this image.
    pub format: U,
}

/// A transformation or orientation in cartesian 2-space.
#[derive(Clone, Copy, Debug)]
pub struct Transform {
    /// Position data.
    pub position: Vector,
    /// Scale data.
    pub scale: Vector,
    /// Rotation data in radians.
    pub rotation: f64,
}

impl Transform {
    /// Sets the position.
    pub fn with_position<T>(mut self, position: T) -> Self
    where
        T: Into<Vector>,
    {
        self.position = position.into();
        self
    }
    /// Sets the scale.
    pub fn with_scale<T>(mut self, scale: T) -> Self
    where
        T: Into<Vector>,
    {
        self.scale = scale.into();
        self
    }
    /// Sets the rotation.
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }
    /// Creates a 3 by 2 matrix of floats representing the first two rows of the
    /// 2-dimensional affine transformation contained in the [Transform].
    pub fn to_matrix(&self) -> [f64; 6] {
        [
            self.scale.x * self.rotation.cos(),
            self.scale.y * self.rotation.sin(),
            -self.scale.x * self.rotation.sin(),
            self.scale.y * self.rotation.cos(),
            self.position.x,
            self.position.y,
        ]
    }
    /// Translates the position by the provided offset.
    pub fn translate<T>(&mut self, offset: T) -> &mut Self
    where
        T: Into<Vector>,
    {
        self.position += offset.into();
        self
    }
    /// Applies a provided additional rotation.
    pub fn rotate(&mut self, rotation: f64) -> &mut Self {
        self.rotation += rotation;
        self
    }
    /// Multiplicatively scales the current scale by that provided.
    pub fn scale<T>(&mut self, scale: T) -> &mut Self
    where
        T: Into<Vector>,
    {
        self.scale *= scale.into();
        self
    }
    /// Composes the transform with another provided transform.
    pub fn transform(&mut self, transform: Transform) -> &mut Self {
        self.scale *= transform.scale;
        self.rotation += transform.rotation;
        self.position += transform.position;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            scale: Vector { x: 1., y: 1. },
            position: Vector::default(),
            rotation: 0.,
        }
    }
}

impl From<Vector> for Transform {
    fn from(input: Vector) -> Transform {
        Transform::default().with_position(input)
    }
}

impl From<(f64, f64)> for Transform {
    fn from(input: (f64, f64)) -> Transform {
        Vector::from(input).into()
    }
}

/// Represents content optimized and cached for rendering.
pub trait Object: Sync + Send {
    /// Composes a transformation with the existing transformation of the [Object].
    fn apply_transform(&mut self, transform: Transform);
    /// Gets the current trasnformation of the [Object].
    fn get_transform(&self) -> Transform;
    /// Sets the current transfomration of the [Object].
    fn set_transform(&mut self, transform: Transform);
    /// Gets the current z-depth of the [Object].
    fn get_depth(&self) -> u32;
    /// Sets the current z-depth of the [Object].
    fn set_depth(&mut self, depth: u32);
    /// Replaces the contents of the [Object] with new Rasterizable content. This may be costly.
    fn update(&mut self, content: Rasterizable);
}

/// An isolated rendering context.
pub trait Frame: Sync + Send {
    /// Adds content to the [Frame].
    fn add(&mut self, content: Content) -> Box<dyn Object>;
    /// Resizes the [Frame]. This does not resize the viewport.
    fn resize(&self, size: Vector);
    /// Sets the viewport.
    fn set_viewport(&self, viewport: Rect);
    /// Returns the size of the [Frame].
    fn get_size(&self) -> Vector;
    /// Returns an image that is a still rasterization of any rendered content.
    fn to_image(&self) -> Box<dyn ImageRepresentation>;
    /// Returns the measured dimensions of some provided content.
    fn measure(&self, input: Rasterizable) -> Vector;
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn Frame>;
    #[doc(hidden)]
    fn show(&self);
    #[doc(hidden)]
    fn draw(&self);
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
    pub(crate) transform: Transform,
}

impl Content {
    /// Sets the orientation of the content.
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    /// Applies a transformation to the content.
    pub fn apply_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
    /// Sets the z-depth of the content.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
}

impl From<Rasterizable> for Content {
    fn from(input: Rasterizable) -> Content {
        Content {
            content: input,
            depth: 0,
            transform: Transform::default(),
        }
    }
}

/// A type that can rasterized.
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
pub trait Rasterizer: Sync + Send {
    /// Returns a rasterization of the input.
    fn rasterize(&self, input: Rasterizable, vector: Vector) -> Box<dyn ImageRepresentation>;
}

/// Provides 2-dimensional euclidean rendering capabilities.
pub trait Graphics: Rasterizer {
    /// Returns a new [Frame].
    fn frame(&self) -> Box<dyn Frame>;
}

/// A post-activation graphics context.
pub trait ContextGraphics: Graphics + Context + Ticker {}

/// An inactive [ContextualGraphics] context.
pub trait InactiveContextGraphics: ContextGraphics {
    /// Begins execution of the runloop. Consumes the context and blocks forever where appropriate.
    fn run(self: Box<Self>, cb: Box<dyn FnMut(Box<dyn ContextGraphics>) + 'static>);
}

/// A type that permits the binding of tick handlers.
pub trait Ticker {
    /// Binds a handler to receive ticks.
    fn bind(&mut self, handler: Box<dyn FnMut(f64) + 'static + Send + Sync>);
}

/// A graphics context that can provide input and windowing.
pub trait ContextualGraphics: Graphics {
    /// Starts a windowed context using the provided [Frame] as the document root.
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveContextGraphics>;
}

/// A 2-dimensional cartesian vector or point
#[derive(Clone, Copy, Default, Debug)]
pub struct Vector {
    /// X-axis position.
    pub x: f64,
    /// Y-axis position.
    pub y: f64,
}

impl From<(f64, f64)> for Vector {
    fn from(input: (f64, f64)) -> Vector {
        Vector {
            x: input.0,
            y: input.1,
        }
    }
}

impl From<f64> for Vector {
    fn from(input: f64) -> Vector {
        Vector { x: input, y: input }
    }
}

impl<T> Add<T> for Vector
where
    T: Into<Vector>,
{
    type Output = Vector;
    fn add(self, other: T) -> Vector {
        let other = other.into();
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign<T> for Vector
where
    T: Into<Vector>,
{
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub<T> for Vector
where
    T: Into<Vector>,
{
    type Output = Vector;
    fn sub(self, other: T) -> Vector {
        let other = other.into();
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign<T> for Vector
where
    T: Into<Vector>,
{
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Div<T> for Vector
where
    T: Into<Vector>,
{
    type Output = Vector;
    fn div(self, other: T) -> Vector {
        let other = other.into();
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign<T> for Vector
where
    T: Into<Vector>,
{
    fn div_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Mul<T> for Vector
where
    T: Into<Vector>,
{
    type Output = Vector;
    fn mul(self, other: T) -> Vector {
        let other = other.into();
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> MulAssign<T> for Vector
where
    T: Into<Vector>,
{
    fn mul_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

/// A rectilinear area of 2-dimensional cartesian space
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    /// The size of the delineated space.
    pub size: Vector,
    /// The position of the origin of the delineated space.
    pub position: Vector,
}

impl Rect {
    /// Creates a new [Rect] from the provided position and size
    pub fn new<T, U>(position: T, size: U) -> Self
    where
        T: Into<Vector>,
        U: Into<Vector>,
    {
        Rect {
            size: size.into(),
            position: position.into(),
        }
    }
}

/// Initializes a new graphics context.
pub fn new() -> Box<dyn ContextualGraphics> {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::graphics::new()
}
