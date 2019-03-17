use crate::input::*;
use crate::path::*;
use crate::targets;
use crate::text::*;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use std::borrow::Cow;

use std::any::Any;

use std::fmt;
use std::fmt::{Debug, Formatter};

/// A conversion to an eight-character hex color string.
pub trait ToHexColor {
    /// Performs the conversion.
    fn to_hex_color(&self) -> Cow<'_, str>;
}

/// A representation type of some target-specific image format.
pub trait ImageRepresentation: Any {
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn ImageRepresentation>;
    /// Returns the 2-d cartesian pixel size of the image.
    fn get_size(&self) -> Vector;
    /// Returns a conversion of the image to [Image<RGBA8, Texture2D>]. This operation may be expensive.
    fn as_texture(&self) -> Image<RGBA8, Texture2D>;
    /// Creates an image in the associated format from an [Image<RGBA8, Texture2D>]. This operation may be expensive.
    fn from_texture(texture: Image<RGBA8, Texture2D>) -> Self
    where
        Self: Sized;
}

impl Clone for Box<dyn ImageRepresentation> {
    fn clone(&self) -> Box<dyn ImageRepresentation> {
        self.box_clone()
    }
}

impl ImageRepresentation for Image<RGBA8, Texture2D> {
    fn get_size(&self) -> Vector {
        (f64::from(self.format.width), f64::from(self.format.height)).into()
    }
    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }
    fn as_texture(&self) -> Image<RGBA8, Texture2D> {
        self.clone()
    }
    fn from_texture(texture: Image<RGBA8, Texture2D>) -> Image<RGBA8, Texture2D> {
        texture
    }
}

/// Indicates that a type is a pixel format for image data.
pub trait PixelFormat {}

/// A standard 24-bit-depth RGB color with an 8-bit alpha channel.
#[derive(Clone, Copy, Debug)]
pub struct RGBA8 {
    /// Red channel data.
    pub r: u8,
    /// Green channel data.
    pub g: u8,
    /// Blue channel data.
    pub b: u8,
    /// Alpha channel data.
    pub a: u8,
}

impl RGBA8 {
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
        RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    /// Returns a fully opaque white color.
    pub fn white() -> Self {
        RGBA8 {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

impl ToHexColor for RGBA8 {
    fn to_hex_color(&self) -> Cow<'_, str> {
        Cow::from(format!(
            "#{:x?}{:x?}{:x?}{:x?}",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl Into<Texture> for RGBA8 {
    fn into(self) -> Texture {
        Texture::Solid(self)
    }
}

impl PixelFormat for RGBA8 {}

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

/// An object with characteristics that may change dynamically in real-time
pub trait DynamicObject {
    /// Returns the absolute orientation of the object within the space of its parent frame.
    fn orientation(&self) -> Transform;
    /// Returns the styled paths comprising the object.
    fn render(&self) -> Cow<'_, [Path]>;
}

/// An object with static contents.
#[derive(Debug)]
pub struct StaticObject {
    /// The absolute orientation of the object within the space of its parent frame.
    pub orientation: Transform,
    /// The styled paths comprising the object.
    pub content: Vec<Path>,
}

impl StaticObject {
    /// Creates a [StaticObject] from a [Path].
    pub fn from_entity(entity: Path) -> StaticObject {
        StaticObject {
            content: vec![entity],
            orientation: Transform::default(),
        }
    }
    /// Passes a mutable reference to the orientation of the object
    /// to the provided closure to permit ergonomic inline
    /// transformation of static objects.
    pub fn with_transform(mut self, closure: impl Fn(&mut Transform)) -> Self {
        closure(&mut self.orientation);
        self
    }
}

impl<T: 'static> From<T> for StaticObject
where
    T: ImageRepresentation,
{
    fn from(input: T) -> Self {
        StaticObject {
            orientation: Transform::default(),
            content: vec![Primitive::rectangle(input.get_size())
                .fill(Texture::Image(Box::new(input)).into())
                .finalize()],
        }
    }
}

impl Into<Object> for StaticObject {
    fn into(self) -> Object {
        Object::Static(Box::new(self))
    }
}

/// An object suitable for rendering.
pub enum Object {
    /// A [StaticObject].
    Static(Box<StaticObject>),
    /// A [DynamicObject].
    Dynamic(Box<dyn DynamicObject>),
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object ( {} )",
            match self {
                Object::Static(object) => format!("Static {:?}", object),
                Object::Dynamic(_) => "Dynamic".to_owned(),
            }
        )
    }
}

/// An isolated rendering context.
pub trait Frame: DynamicObject + Clone {
    /// The [ImageRepresentation] used internally by the [Frame].
    type Image: ImageRepresentation;
    /// Adds content to the [Frame].
    fn add<U>(&mut self, object: U)
    where
        U: Into<Object>;
    /// Resizes the [Frame]. This does not resize the viewport.
    fn resize<U>(&self, size: U)
    where
        U: Into<Vector>;
    /// Sets the viewport.
    fn set_viewport(&self, viewport: Rect);
    /// Returns the size of the [Frame].
    fn get_size(&self) -> Vector;
    /// Returns an image that is a still rasterization of any rendered content.
    fn to_image(&self) -> Box<<Self as Frame>::Image>;
}

/// A type that can rasterized.
#[derive(Debug)]
pub enum Rasterizable<'a> {
    /// Some [Text].
    Text(Text<'a>),
}

/// Provides an interface for the rasterization of content.
pub trait Rasterizer {
    /// The image representation type used.
    type Image: ImageRepresentation;
    /// Returns a rasterization of the input.
    fn rasterize<'a, T>(&self, input: T) -> Self::Image
    where
        T: Into<Rasterizable<'a>>;
}

/// Provides 2-dimensional euclidean rendering capabilities.
pub trait Graphics: Rasterizer {
    /// The internal concrete type of the [Frame]s used.
    type Frame: Frame;
    /// Returns a new [Frame].
    fn frame(&self) -> Self::Frame;
}

/// An active [ContextualGraphics] context.
pub trait ContextGraphics: Graphics + Context {}

/// A graphics context that can provide input and windowing.
pub trait ContextualGraphics: Graphics {
    /// The internal concrete type of the [Context] returned upon activation.
    type Context: ContextGraphics;
    /// Starts a windowed context using the provided [Frame] as the document root.
    fn run(self, root: Self::Frame) -> Self::Context;
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
pub fn new() -> impl ContextualGraphics {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::graphics::new()
}
