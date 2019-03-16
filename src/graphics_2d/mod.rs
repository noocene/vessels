use crate::input::*;
use crate::path::*;
use crate::targets;
use crate::text::*;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use std::borrow::Cow;

pub trait ToHexColor {
    fn to_hex_color(&self) -> Cow<str>;
}

pub trait ImageRepresentation:
    From<Image<RGBA8, Texture2D>> + Into<Image<RGBA8, Texture2D>> + Clone
{
    fn get_size(&self) -> Vector;
}

impl ImageRepresentation for Image<RGBA8, Texture2D> {
    fn get_size(&self) -> Vector {
        (f64::from(self.format.width), f64::from(self.format.height)).into()
    }
}

pub trait PixelFormat {}

#[derive(Clone)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA8 {
    pub fn to_rgba_color(&self) -> Cow<str> {
        Cow::from(format!(
            "rgba({},{},{},{})",
            self.r,
            self.g,
            self.b,
            f64::from(self.a) / 255.
        ))
    }
    pub fn with_alpha(mut self, alpha: u8) -> Self {
        self.a = alpha;
        self
    }
    pub fn black() -> Self {
        RGBA8 {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
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
    fn to_hex_color(&self) -> Cow<str> {
        Cow::from(format!(
            "#{:x?}{:x?}{:x?}{:x?}",
            self.r, self.g, self.b, self.a
        ))
    }
}

impl<T> Into<Texture<T>> for RGBA8
where
    T: ImageRepresentation,
{
    fn into(self) -> Texture<T> {
        Texture::Solid(self)
    }
}

impl PixelFormat for RGBA8 {}

pub trait ImageFormat {}

#[derive(Clone)]
pub struct Texture2D {
    pub width: u32,
    pub height: u32,
}

impl ImageFormat for Texture2D {}

#[derive(Clone)]
pub struct Image<T: PixelFormat, U: ImageFormat> {
    pub pixels: Vec<T>,
    pub format: U,
}

#[derive(Clone)]
pub struct Transform {
    pub position: Vector,
    pub scale: Vector,
    pub rotation: f64,
}

impl Transform {
    pub fn with_position<T>(mut self, position: T) -> Self
    where
        T: Into<Vector>,
    {
        self.position = position.into();
        self
    }
    pub fn with_scale<T>(mut self, scale: T) -> Self
    where
        T: Into<Vector>,
    {
        self.scale = scale.into();
        self
    }
    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
    }
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
    pub fn translate<T>(&mut self, offset: T) -> &mut Self
    where
        T: Into<Vector>,
    {
        self.position += offset.into();
        self
    }
    pub fn rotate(&mut self, rotation: f64) -> &mut Self {
        self.rotation += rotation;
        self
    }
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

pub trait DynamicObject {
    type Image: ImageRepresentation;
    fn orientation(&self) -> Transform;
    fn render(&self) -> Cow<[Path<Self::Image>]>;
}

pub struct StaticObject<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform,
    pub content: Vec<Path<T>>,
}

impl<T> StaticObject<T>
where
    T: ImageRepresentation,
{
    pub fn from_entity(entity: Path<T>) -> StaticObject<T> {
        StaticObject {
            content: vec![entity],
            orientation: Transform::default(),
        }
    }
    pub fn with_transform(mut self, closure: impl Fn(&mut Transform)) -> Self {
        closure(&mut self.orientation);
        self
    }
}

impl<T> From<T> for StaticObject<T>
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

impl<T> Into<Object<T>> for StaticObject<T>
where
    T: ImageRepresentation,
{
    fn into(self) -> Object<T> {
        Object::Static(self)
    }
}

pub enum Object<T>
where
    T: ImageRepresentation,
{
    Static(StaticObject<T>),
    Dynamic(Box<DynamicObject<Image = T>>),
}

pub trait Frame: DynamicObject<Image = <Self as Frame>::Image> {
    type Image: ImageRepresentation;
    fn add<U>(&mut self, object: U)
    where
        U: Into<Object<<Self as Frame>::Image>>;
    fn resize<U>(&self, size: U)
    where
        U: Into<Vector>;
    fn set_viewport(&self, viewport: Rect);
    fn get_size(&self) -> Vector;
    fn to_image(&self) -> Box<<Self as Frame>::Image>;
}

pub enum Rasterizable<'a> {
    Text(Text<'a>),
}

pub trait Rasterizer {
    type Image: ImageRepresentation;
    fn rasterize<'a, T>(&self, input: T) -> Self::Image
    where
        T: Into<Rasterizable<'a>>;
}

pub trait Graphics: Rasterizer {
    type Frame: Frame;
    fn frame(&self) -> Self::Frame;
}

pub trait ContextGraphics: Graphics + Context {}

pub trait ContextualGraphics: Graphics {
    type Context: ContextGraphics;
    fn run(self, root: Self::Frame) -> Self::Context;
}

#[derive(Clone, Copy, Default)]
pub struct Vector {
    pub x: f64,
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

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub size: Vector,
    pub position: Vector,
}

impl Rect {
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

pub fn new() -> impl ContextualGraphics {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
    targets::web::graphics::new()
}
