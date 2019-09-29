use std::{
    any::Any,
    borrow::Cow,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Provides bindings to 2D vector graphic rendering functionality.
pub mod canvas;
/// Provides helper types that allow ergonomic construction and styling of 2D vector graphics.
pub mod path;
/// Contains types to help represent and construct styled text.
pub mod text;

/// A conversion to an eight-character hex color string.
pub trait ToHexLDRColor {
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
    fn get_size(&self) -> Vector2;
    /// Returns a conversion of the image to [Image<LDRColor, Texture2>]. This operation may be expensive.
    fn as_texture(&self) -> Image<LDRColor, Texture2>;
    /// Creates an image in the associated format from an [Image<LDRColor, Texture2>]. This operation may be expensive.
    fn from_texture(texture: Image<LDRColor, Texture2>) -> Self
    where
        Self: Sized;
}

impl Clone for Box<dyn ImageRepresentation> {
    fn clone(&self) -> Box<dyn ImageRepresentation> {
        self.box_clone()
    }
}

impl ImageRepresentation for Image<LDRColor, Texture2> {
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
    fn get_size(&self) -> Vector2 {
        (f64::from(self.format.width), f64::from(self.format.height)).into()
    }
    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }
    fn as_texture(&self) -> Image<LDRColor, Texture2> {
        self.clone()
    }
    fn from_texture(texture: Image<LDRColor, Texture2>) -> Image<LDRColor, Texture2> {
        texture
    }
}

/// Indicates that a type is a pixel format for image data.
pub trait PixelFormat {}

/// A standard 24-bit-depth LDR sRGB color with 8-bit alpha channel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LDRColor {
    /// Red channel data.
    pub r: u8,
    /// Green channel data.
    pub g: u8,
    /// Blue channel data.
    pub b: u8,
    /// Alpha channel data.
    pub a: u8,
}

impl From<(u8, u8, u8)> for LDRColor {
    fn from(input: (u8, u8, u8)) -> LDRColor {
        LDRColor {
            r: input.0,
            g: input.1,
            b: input.2,
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for LDRColor {
    fn from(input: (u8, u8, u8, u8)) -> LDRColor {
        LDRColor {
            r: input.0,
            g: input.1,
            b: input.2,
            a: input.3,
        }
    }
}

impl LDRColor {
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
        LDRColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
    /// Returns a fully opaque white color.
    pub fn white() -> Self {
        LDRColor {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
    /// Creates a new fully opaque color from the provided RGB values.
    pub fn rgb(r: u8, g: u8, b: u8) -> LDRColor {
        LDRColor { r, g, b, a: 255 }
    }
    /// Creates a new opaque color from the provided RGBA values.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> LDRColor {
        LDRColor { r, g, b, a }
    }
}

impl ToHexLDRColor for LDRColor {
    fn to_hex_color(&self) -> Cow<'_, str> {
        Cow::from(format!(
            "#{:x?}{:x?}{:x?}{:x?}",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl PixelFormat for LDRColor {}

/// Indicates that a type is an organizational format for image data.
pub trait ImageFormat {}

/// A typical two-dimensional grid image format with square pixels.
#[derive(Clone, Copy, Debug)]
pub struct Texture2 {
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
}

impl ImageFormat for Texture2 {}

/// A concrete image composed of format data and a flat [Vec] of pixels
#[derive(Clone, Debug)]
pub struct Image<T: PixelFormat, U: ImageFormat> {
    /// Pixel data.
    pub pixels: Vec<T>,
    /// Format of this image.
    pub format: U,
}

/// A 2-dimensional cartesian vector or point
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vector2 {
    /// X-axis position.
    pub x: f64,
    /// Y-axis position.
    pub y: f64,
}

impl From<(f64, f64)> for Vector2 {
    fn from(input: (f64, f64)) -> Vector2 {
        Vector2 {
            x: input.0,
            y: input.1,
        }
    }
}

impl From<f64> for Vector2 {
    fn from(input: f64) -> Vector2 {
        Vector2 { x: input, y: input }
    }
}

impl<T> Add<T> for Vector2
where
    T: Into<Vector2>,
{
    type Output = Vector2;
    fn add(self, other: T) -> Vector2 {
        let other = other.into();
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign<T> for Vector2
where
    T: Into<Vector2>,
{
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub<T> for Vector2
where
    T: Into<Vector2>,
{
    type Output = Vector2;
    fn sub(self, other: T) -> Vector2 {
        let other = other.into();
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> SubAssign<T> for Vector2
where
    T: Into<Vector2>,
{
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Div<T> for Vector2
where
    T: Into<Vector2>,
{
    type Output = Vector2;
    fn div(self, other: T) -> Vector2 {
        let other = other.into();
        Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> DivAssign<T> for Vector2
where
    T: Into<Vector2>,
{
    fn div_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl<T> Mul<T> for Vector2
where
    T: Into<Vector2>,
{
    type Output = Vector2;
    fn mul(self, other: T) -> Vector2 {
        let other = other.into();
        Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> MulAssign<T> for Vector2
where
    T: Into<Vector2>,
{
    fn mul_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vector2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

/// A rectilinear area of 2-dimensional cartesian space
#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    /// The size of the delineated space.
    pub size: Vector2,
    /// The position of the origin of the delineated space.
    pub position: Vector2,
}

impl Rect {
    /// Creates a new [Rect] from the provided position and size
    pub fn new<T, U>(position: T, size: U) -> Self
    where
        T: Into<Vector2>,
        U: Into<Vector2>,
    {
        Rect {
            size: size.into(),
            position: position.into(),
        }
    }
}

/// A transformation or orientation in cartesian 2-space.
#[derive(Clone, Copy, Debug)]
pub struct Transform2 {
    /// Position data.
    pub position: Vector2,
    /// Scale data.
    pub scale: Vector2,
    /// Rotation data in radians.
    pub rotation: f64,
}

impl Transform2 {
    /// Sets the position.
    pub fn with_position<T>(mut self, position: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.position = position.into();
        self
    }
    /// Sets the scale.
    pub fn with_scale<T>(mut self, scale: T) -> Self
    where
        T: Into<Vector2>,
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
    /// 2-dimensional affine transformation contained in the [Transform2].
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
        T: Into<Vector2>,
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
        T: Into<Vector2>,
    {
        self.scale *= scale.into();
        self
    }
    /// Composes the transform with another provided transform.
    pub fn transform(&mut self, transform: Transform2) -> &mut Self {
        self.scale *= transform.scale;
        self.rotation += transform.rotation;
        self.position += transform.position;
        self
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Transform2 {
            scale: Vector2 { x: 1., y: 1. },
            position: Vector2::default(),
            rotation: 0.,
        }
    }
}

impl From<Vector2> for Transform2 {
    fn from(input: Vector2) -> Transform2 {
        Transform2::default().with_position(input)
    }
}

impl From<(f64, f64)> for Transform2 {
    fn from(input: (f64, f64)) -> Transform2 {
        Vector2::from(input).into()
    }
}
