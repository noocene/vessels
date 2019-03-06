use crate::util::*;

const CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO: f64 = 0.552_228_474;

use std::borrow::Cow;

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

impl<T> Into<VectorEntity2DFill<T>> for RGBA8
where
    T: ImageRepresentation,
{
    fn into(self) -> VectorEntity2DFill<T> {
        VectorEntity2DFill {
            content: self.into(),
        }
    }
}

impl<T> Into<VectorEntityTexture<T>> for RGBA8
where
    T: ImageRepresentation,
{
    fn into(self) -> VectorEntityTexture<T> {
        VectorEntityTexture::Solid(self)
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
pub enum VectorEntity2DSegment {
    LineTo(Point2D),
    MoveTo(Point2D),
    QuadraticTo(Point2D, Point2D),
    CubicTo(Point2D, Point2D, Point2D),
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
pub struct VectorEntity2DShadow {
    pub color: RGBA8,
    pub offset: Distance2D,
    pub blur: f64,
}

impl VectorEntity2DShadow {
    pub fn new(color: RGBA8) -> Self {
        VectorEntity2DShadow {
            color,
            offset: Distance2D::default(),
            blur: 0.,
        }
    }
    pub fn blur(mut self, amount: f64) -> Self {
        self.blur = amount;
        self
    }
    pub fn offset(mut self, distance: Distance2D) -> Self {
        self.offset = distance;
        self
    }
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
pub enum VectorEntityTexture<T>
where
    T: ImageRepresentation,
{
    Solid(RGBA8),
    LinearGradient(VectorEntityLinearGradient),
    RadialGradient(VectorEntityRadialGradient),
    Image(Box<T>),
}

#[derive(Clone)]
pub struct VectorEntity2DStroke<T>
where
    T: ImageRepresentation,
{
    pub content: VectorEntityTexture<T>,
    pub width: f32,
    pub cap: StrokeCapType,
    pub join: StrokeJoinType,
}

impl<T> Default for VectorEntity2DStroke<T>
where
    T: ImageRepresentation,
{
    fn default() -> Self {
        VectorEntity2DStroke {
            content: RGBA8::black().into(),
            cap: StrokeCapType::Butt,
            join: StrokeJoinType::Miter,
            width: 1.,
        }
    }
}

#[derive(Clone)]
pub struct VectorEntity2DFill<T>
where
    T: ImageRepresentation,
{
    pub content: VectorEntityTexture<T>,
}

#[derive(Clone)]
pub struct Entity2D<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform2D,
    pub segments: Vec<VectorEntity2DSegment>,
    pub stroke: Option<VectorEntity2DStroke<T>>,
    pub fill: Option<VectorEntity2DFill<T>>,
    pub shadow: Option<VectorEntity2DShadow>,
    pub closed: bool,
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
    fn render(&self) -> Cow<[Entity2D<T>]>;
}

pub struct StaticObject2D<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform2D,
    pub content: Vec<Entity2D<T>>,
}

impl<T> StaticObject2D<T>
where
    T: ImageRepresentation,
{
    pub fn from_entity(entity: Entity2D<T>) -> Object2D<T> {
        Object2D::Static(StaticObject2D {
            content: vec![entity],
            orientation: Transform2D::default(),
        })
    }
}

#[derive(Default)]
pub struct VectorGeometryBuilder {
    segments: Vec<VectorEntity2DSegment>,
}

impl VectorGeometryBuilder {
    pub fn new() -> Self {
        VectorGeometryBuilder::default()
    }
    pub fn line_to(mut self, to: Point2D) -> Self {
        self.segments.push(VectorEntity2DSegment::LineTo(to));
        self
    }
    pub fn quadratic_to(mut self, to: Point2D, handle: Point2D) -> Self {
        self.segments
            .push(VectorEntity2DSegment::QuadraticTo(to, handle));
        self
    }
    pub fn bezier_to(mut self, to: Point2D, handle_1: Point2D, handle_2: Point2D) -> Self {
        self.segments
            .push(VectorEntity2DSegment::CubicTo(to, handle_1, handle_2));
        self
    }
    pub fn done<T>(self) -> VectorEntityBuilder<T>
    where
        T: ImageRepresentation,
    {
        VectorEntityBuilder::new(self.segments)
    }
}

pub struct VectorGeometryPrimitive {}

impl VectorGeometryPrimitive {
    pub fn rectangle<T>(width: f64, height: f64) -> VectorEntityBuilder<T>
    where
        T: ImageRepresentation,
    {
        VectorEntityBuilder::new(vec![
            VectorEntity2DSegment::LineTo(Point2D::new(width, 0.)),
            VectorEntity2DSegment::LineTo(Point2D::new(width, height)),
            VectorEntity2DSegment::LineTo(Point2D::new(0., height)),
        ])
    }
    pub fn rounded_rectangle<T>(width: f64, height: f64, radius: f64) -> VectorEntityBuilder<T>
    where
        T: ImageRepresentation,
    {
        VectorEntityBuilder::new(vec![
            VectorEntity2DSegment::MoveTo(Point2D::new(radius, 0.)),
            VectorEntity2DSegment::LineTo(Point2D::new(width - radius, 0.)),
            VectorEntity2DSegment::CubicTo(
                Point2D::new(width, radius),
                Point2D::new(
                    width - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    0.,
                ),
                Point2D::new(
                    width,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            ),
            VectorEntity2DSegment::LineTo(Point2D::new(width, height - radius)),
            VectorEntity2DSegment::CubicTo(
                Point2D::new(width - radius, height),
                Point2D::new(
                    width,
                    height - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
                Point2D::new(
                    width - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    height,
                ),
            ),
            VectorEntity2DSegment::LineTo(Point2D::new(radius, height)),
            VectorEntity2DSegment::CubicTo(
                Point2D::new(0., height - radius),
                Point2D::new(
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    height,
                ),
                Point2D::new(
                    0.,
                    height - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            ),
            VectorEntity2DSegment::LineTo(Point2D::new(0., radius)),
            VectorEntity2DSegment::CubicTo(
                Point2D::new(radius, 0.),
                Point2D::new(0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)),
                Point2D::new(radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.),
            ),
        ])
    }
    pub fn square<T>(side_length: f64) -> VectorEntityBuilder<T>
    where
        T: ImageRepresentation,
    {
        VectorGeometryPrimitive::rectangle(side_length, side_length)
    }
    pub fn rounded_square<T>(side_length: f64, radius: f64) -> VectorEntityBuilder<T>
    where
        T: ImageRepresentation,
    {
        VectorGeometryPrimitive::rounded_rectangle(side_length, side_length, radius)
    }
}

pub struct VectorEntityBuilder<T>
where
    T: ImageRepresentation,
{
    closed: bool,
    geometry: Vec<VectorEntity2DSegment>,
    fill: Option<VectorEntity2DFill<T>>,
    stroke: Option<VectorEntity2DStroke<T>>,
    shadow: Option<VectorEntity2DShadow>,
}

impl<T> VectorEntityBuilder<T>
where
    T: ImageRepresentation,
{
    pub fn new(geometry: Vec<VectorEntity2DSegment>) -> Self {
        VectorEntityBuilder {
            closed: false,
            geometry,
            fill: None,
            shadow: None,
            stroke: None,
        }
    }
    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }
    pub fn fill(mut self, fill: VectorEntity2DFill<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, stroke: VectorEntity2DStroke<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.stroke = Some(stroke);
        self
    }
    pub fn shadow(mut self, shadow: VectorEntity2DShadow) -> Self
    where
        T: ImageRepresentation,
    {
        self.shadow = Some(shadow);
        self
    }
    pub fn finalize(self) -> Entity2D<T>
    where
        T: ImageRepresentation,
    {
        Entity2D {
            closed: self.closed,
            segments: self.geometry,
            orientation: Transform2D::default(),
            fill: self.fill,
            shadow: self.shadow,
            stroke: self.stroke,
        }
    }
}

pub struct StrokeBuilder<T>
where
    T: ImageRepresentation,
{
    stroke: VectorEntity2DStroke<T>,
}

impl<T> StrokeBuilder<T>
where
    T: ImageRepresentation,
{
    pub fn new(content: VectorEntityTexture<T>, width: f32) -> Self {
        let mut builder = StrokeBuilder {
            stroke: VectorEntity2DStroke::default(),
        };
        builder.stroke.content = content;
        builder.stroke.width = width;
        builder
    }
    pub fn cap_round(mut self) -> Self {
        self.stroke.cap = StrokeCapType::Round;
        self
    }
    pub fn join_bevel(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Bevel;
        self
    }
    pub fn join_round(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Round;
        self
    }
    pub fn finalize(self) -> VectorEntity2DStroke<T> {
        self.stroke
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
