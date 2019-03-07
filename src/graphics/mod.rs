use crate::graphics::path::*;
use crate::util::*;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Div, DivAssign};

use std::borrow::Cow;

pub mod path;

pub trait ToHexColor {
    fn to_hex_color(&self) -> String;
}

pub trait ImageRepresentation:
    From<Image<RGBA8, Texture2D>> + Into<Image<RGBA8, Texture2D>> + Clone
{
}

impl ImageRepresentation for Image<RGBA8, Texture2D> {}

pub trait PixelFormat {}

#[derive(Clone)]
pub struct RGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA8 {
    pub fn to_rgba_color(&self) -> String {
        format!(
            "rgba({},{},{},{})",
            self.r,
            self.g,
            self.b,
            f64::from(self.a) / 255.
        )
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
    fn to_hex_color(&self) -> String {
        format!("#{:x?}{:x?}{:x?}{:x?}", self.r, self.g, self.b, self.a)
    }
}

impl<T> Into<Fill<T>> for RGBA8
where
    T: ImageRepresentation,
{
    fn into(self) -> Fill<T> {
        Fill {
            content: self.into(),
        }
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
    pub height: u32
}

impl ImageFormat for Texture2D {}

#[derive(Clone)]
pub struct Image<T: PixelFormat, U: ImageFormat> {
    pub pixels: Vec<T>,
    pub format: U,
}

#[derive(Clone)]
pub struct Transform2D {
    pub position: Vec2D,
    pub scale: Vec2D,
    pub rotation: f64,
}

impl Transform2D {
    pub fn with_position<T>(mut self, position: T) -> Self where T: Into<Vec2D> {
        self.position = position.into();
        self
    }
    pub fn with_scale<T>(mut self, scale: T) -> Self where T: Into<Vec2D> {
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
    pub fn translate<T>(&mut self, offset: T) -> &mut Self where T: Into<Vec2D> {
        self.position += offset.into();
        self
    }
    pub fn rotate(&mut self, rotation: f64) -> &mut Self {
        self.rotation += rotation;
        self
    }
    pub fn scale<T>(&mut self, scale: T) -> &mut Self where T: Into<Vec2D> {
        self.scale *= scale.into();
        self
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Transform2D {
            scale: Vec2D { x: 1., y: 1. },
            position: Vec2D::default(),
            rotation: 0.,
        }
    }
}

pub trait DynamicObject2D<T>
where
    T: ImageRepresentation,
{
    fn orientation(&self) -> Transform2D;
    fn render(&self) -> Cow<[Path<T>]>;
}

pub struct StaticObject2D<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform2D,
    pub content: Vec<Path<T>>,
}

impl<T> StaticObject2D<T>
where
    T: ImageRepresentation,
{
    pub fn from_entity(entity: Path<T>) -> Object2D<T> {
        Object2D::Static(StaticObject2D {
            content: vec![entity],
            orientation: Transform2D::default(),
        })
    }
}

pub enum Object2D<T>
where
    T: ImageRepresentation,
{
    Static(StaticObject2D<T>),
    Dynamic(Box<DynamicObject2D<T>>),
}

pub trait Frame2D<T>: DynamicObject2D<T>
where
    T: ImageRepresentation,
{
    fn add(&mut self, object: Object2D<T>);
    fn resize<U>(&self, size: U) where U: Into<Vec2D>;
    fn set_viewport(&self, viewport: Rect2D);
    fn get_size(&self) -> Vec2D;
    fn to_image(&self) -> Box<T>;
}

pub trait Graphics2D {
    type Image: ImageRepresentation;
    type Frame: Frame2D<Self::Image>;
    fn run(self, root: Self::Frame);
    fn frame(&self) -> Self::Frame;
}

#[derive(Clone, Copy, Default)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Vec2D {
    fn from(input: (f64, f64)) -> Vec2D {
        Vec2D{
            x: input.0, y: input.1
        }
    }
}

impl<T> Add<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;
    fn add(self, other: T) -> Vec2D {
        let other = other.into();
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> AddAssign<T> for Vec2D where T: Into<Vec2D> {
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vec2D {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> Sub<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;
    fn sub(self, other: T) -> Vec2D {
        let other = other.into();
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T> SubAssign<T> for Vec2D where T: Into<Vec2D> {
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vec2D {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T> Div<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;
    fn div(self, other: T) -> Vec2D {
        let other = other.into();
        Vec2D {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl<T> DivAssign<T> for Vec2D where T: Into<Vec2D> {
    fn div_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vec2D {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl<T> Mul<T> for Vec2D where T: Into<Vec2D> {
    type Output = Vec2D;
    fn mul(self, other: T) -> Vec2D {
        let other = other.into();
        Vec2D {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<T> MulAssign<T> for Vec2D where T: Into<Vec2D> {
    fn mul_assign(&mut self, other: T) {
        let other = other.into();
        *self = Vec2D {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect2D {
    pub size: Vec2D,
    pub position: Vec2D,
}

impl Rect2D {
    pub fn new<T, U>(position: T, size: U) -> Self where T: Into<Vec2D>, U: Into<Vec2D> {
        Rect2D {
            size: size.into(),
            position: position.into()
        }
    }
}

mod targets;

pub fn new() -> impl Graphics2D {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
    targets::web::new()
}
