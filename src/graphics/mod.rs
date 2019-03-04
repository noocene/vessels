use crate::util::*;

use std::borrow::Cow;

pub trait AsHexColor {
    fn as_hex_color(&self) -> String;
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

impl AsHexColor for RGBA8 {
    fn as_hex_color(&self) -> String {
        format!("#{:x?}{:x?}{:x?}{:x?}", self.r, self.g, self.b, self.a)
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
pub struct RasterEntity2D<'a, T>
where
    T: ImageRepresentation,
{
    pub texture: &'a T,
}

#[derive(Clone)]
pub enum VectorEntity2DSegment {
    Point(Point2D),
    Curve(Point2D, Point2D),
}

#[derive(Clone)]
pub struct VectorEntity2DStroke {
    pub color: RGBA8,
    pub width: u16,
}

#[derive(Clone)]
pub struct VectorEntity2DFill {
    pub color: RGBA8,
}

#[derive(Clone)]
pub struct VectorEntity2D {
    pub segments: Vec<VectorEntity2DSegment>,
    pub stroke: Option<VectorEntity2DStroke>,
    pub fill: Option<VectorEntity2DFill>,
    pub closed: bool,
}

#[derive(Clone)]
pub struct Entity2D<'a, T>
where
    T: ImageRepresentation,
{
    pub offset: Distance2D,
    pub representation: EntityFormat2D<'a, T>,
}

#[derive(Clone)]
pub enum EntityFormat2D<'a, T>
where
    T: ImageRepresentation,
{
    VectorEntity2D(VectorEntity2D),
    RasterEntity2D(RasterEntity2D<'a, T>),
}

pub trait Object2D<T>
where
    T: ImageRepresentation,
{
    fn position(&self) -> Point2D;
    fn render(&self) -> Cow<[Entity2D<T>]>;
}

pub struct ConcreteObject2D<'a, T>
where
    T: ImageRepresentation,
{
    pub position: Point2D,
    pub contents: Vec<Entity2D<'a, T>>,
}

impl<'a, T> Object2D<T> for ConcreteObject2D<'a, T>
where
    T: ImageRepresentation,
{
    fn position(&self) -> Point2D {
        self.position
    }
    fn render(&self) -> Cow<[Entity2D<T>]> {
        Cow::from(&self.contents)
    }
}

pub trait Frame2D<T>: Object2D<T>
where
    T: ImageRepresentation,
{
    fn add(&mut self, object: Box<Object2D<T>>);
    fn resize(&self, size: Size2D);
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

pub type Distance2D = Point2D;

mod targets;

pub fn new() -> impl Graphics2D {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
    targets::web::new()
}
