use crate::graphics::path::*;
use crate::util::*;

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
    pub height: u32,
}

impl ImageFormat for Texture2D {}

#[derive(Clone)]
pub struct Image<T: PixelFormat, U: ImageFormat> {
    pub pixels: Vec<T>,
    pub shape: U,
}

#[derive(Clone)]
pub struct Transform2D {
    pub position: Point2D,
    pub scale: Scale2D,
    pub rotation: Rotation2D,
}

impl Transform2D {
    pub fn with_position(mut self, position: Point2D) -> Self {
        self.position = position;
        self
    }
    pub fn with_scale(mut self, scale: Scale2D) -> Self {
        self.scale = scale;
        self
    }
    pub fn with_rotation(mut self, rotation: Rotation2D) -> Self {
        self.rotation = rotation;
        self
    }
    pub fn to_matrix(&self) -> [f64; 6] {
        [
            self.scale.x,
            0.,
            0.,
            self.scale.y,
            self.position.x,
            self.position.y,
        ]
    }
    pub fn translate(&mut self, x: f64, y: f64) {
        self.position.x += x;
        self.position.y += y;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Transform2D {
            scale: Scale2D { x: 1., y: 1. },
            position: Point2D::default(),
            rotation: Rotation2D::default(),
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
    fn resize(&self, size: Size2D);
    fn set_viewport(&self, viewport: Rect2D);
    fn get_size(&self) -> Size2D;
    fn to_image(&self) -> Box<T>;
}

pub trait Graphics2D {
    type Image: ImageRepresentation;
    type Frame: Frame2D<Self::Image>;
    fn run(self, root: Self::Frame);
    fn frame(&self) -> Self::Frame;
}

#[derive(Clone, Copy, Default)]
pub struct Size2D {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Default)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Rect2D {
    pub size: Size2D,
    pub position: Point2D,
}

impl Rect2D {
    pub fn new(width: f64, height: f64, x: f64, y: f64) -> Self {
        Rect2D {
            size: Size2D { width, height },
            position: Point2D { x, y },
        }
    }
}

pub type Distance2D = Point2D;

pub type Rotation2D = Point2D;

pub type Scale2D = Point2D;

mod targets;

pub fn new() -> impl Graphics2D {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
    targets::web::new()
}
