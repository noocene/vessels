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

impl RGBA8 {
    pub fn as_rgba_color(&self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }
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
pub struct RasterEntity2D<T>
where
    T: ImageRepresentation,
{
    pub texture: Box<T>,
}

#[derive(Clone)]
pub enum VectorEntity2DSegment {
    Point(Point2D),
    Curve(Point2D, Point2D),
}

#[derive(Clone)]
pub enum StrokeCapType {
    Butt,
    Round,
}

#[derive(Clone)]
pub enum StrokeJoinType {
    Bevel,
    Round,
    Miter,
}

#[derive(Clone)]
pub struct VectorEntityGradientStop {
    pub offset: f64,
    pub color: RGBA8,
}

#[derive(Clone)]
pub struct VectorEntityLinearGradient {
    pub stops: Vec<VectorEntityGradientStop>,
    pub start: Point2D,
    pub end: Point2D,
}

#[derive(Clone)]
pub struct VectorEntityRadialGradient {
    pub stops: Vec<VectorEntityGradientStop>,
    pub start: Point2D,
    pub start_radius: f64,
    pub end: Point2D,
    pub end_radius: f64,
}

#[derive(Clone)]
pub enum VectorEntityColor {
    Solid(RGBA8),
    LinearGradient(VectorEntityLinearGradient),
    RadialGradient(VectorEntityRadialGradient),
}

#[derive(Clone)]
pub struct VectorEntity2DStroke {
    pub color: VectorEntityColor,
    pub width: u16,
    pub cap: StrokeCapType,
    pub join: StrokeJoinType,
}

#[derive(Clone)]
pub struct VectorEntity2DFill {
    pub color: VectorEntityColor,
}

#[derive(Clone)]
pub struct VectorEntity2D {
    pub segments: Vec<VectorEntity2DSegment>,
    pub stroke: Option<VectorEntity2DStroke>,
    pub fill: Option<VectorEntity2DFill>,
    pub closed: bool,
}

#[derive(Clone)]
pub struct Orientation2D {
    position: Point2D,
    scale: Size2D,
    rotation: Rotation2D,
}

impl Default for Orientation2D {
    fn default() -> Self {
        Orientation2D {
            scale: Size2D {
                width: 1.,
                height: 1.,
            },
            position: Point2D::default(),
            rotation: Rotation2D::default(),
        }
    }
}

#[derive(Clone)]
pub struct Entity2D<T>
where
    T: ImageRepresentation,
{
    pub orientation: Orientation2D,
    pub representation: EntityFormat2D<T>,
}

#[derive(Clone)]
pub enum EntityFormat2D<T>
where
    T: ImageRepresentation,
{
    VectorEntity2D(VectorEntity2D),
    RasterEntity2D(RasterEntity2D<T>),
}

pub trait DynamicObject2D<T>
where
    T: ImageRepresentation,
{
    fn orientation(&self) -> Orientation2D;
    fn render(&self) -> Cow<[Entity2D<T>]>;
}

pub struct StaticObject2D<T>
where
    T: ImageRepresentation,
{
    pub orientation: Orientation2D,
    pub content: Vec<Entity2D<T>>,
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
    fn get_size(&self) -> Size2D;
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

pub type Rotation2D = Point2D;

mod targets;

pub fn new() -> impl Graphics2D {
    #[cfg(any(target_arch = "wasm32", target_arch = "asmjs", feature = "check"))]
    targets::web::new()
}
