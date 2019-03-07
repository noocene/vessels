use crate::graphics::*;

const CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO: f64 = 0.552_228_474;

#[derive(Clone)]
pub enum Segment {
    LineTo(Vec2D),
    MoveTo(Vec2D),
    QuadraticTo(Vec2D, Vec2D),
    CubicTo(Vec2D, Vec2D, Vec2D),
}

#[derive(Clone)]
pub struct GradientStop {
    pub offset: f64,
    pub color: RGBA8,
}

#[derive(Clone)]
pub struct LinearGradient {
    pub stops: Vec<GradientStop>,
    pub start: Vec2D,
    pub end: Vec2D,
}

#[derive(Clone)]
pub struct Shadow {
    pub color: RGBA8,
    pub offset: Vec2D,
    pub blur: f64,
}

impl Shadow {
    pub fn new(color: RGBA8) -> Self {
        Shadow {
            color,
            offset: Vec2D::default(),
            blur: 0.,
        }
    }
    pub fn blur(mut self, amount: f64) -> Self {
        self.blur = amount;
        self
    }
    pub fn offset(mut self, distance: Vec2D) -> Self {
        self.offset = distance;
        self
    }
}

#[derive(Clone)]
pub struct RadialGradient {
    pub stops: Vec<GradientStop>,
    pub start: Vec2D,
    pub start_radius: f64,
    pub end: Vec2D,
    pub end_radius: f64,
}

#[derive(Clone)]
pub enum Texture<T>
where
    T: ImageRepresentation,
{
    Solid(RGBA8),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Image(Box<T>),
}

#[derive(Clone)]
pub struct Stroke<T>
where
    T: ImageRepresentation,
{
    pub content: Texture<T>,
    pub width: f32,
    pub cap: StrokeCapType,
    pub join: StrokeJoinType,
}

impl<T> Default for Stroke<T>
where
    T: ImageRepresentation,
{
    fn default() -> Self {
        Stroke {
            content: RGBA8::black().into(),
            cap: StrokeCapType::Butt,
            join: StrokeJoinType::Miter,
            width: 1.,
        }
    }
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
pub struct Fill<T>
where
    T: ImageRepresentation,
{
    pub content: Texture<T>,
}

#[derive(Clone)]
pub struct Path<T>
where
    T: ImageRepresentation,
{
    pub orientation: Transform2D,
    pub segments: Vec<Segment>,
    pub stroke: Option<Stroke<T>>,
    pub fill: Option<Fill<T>>,
    pub shadow: Option<Shadow>,
    pub closed: bool,
}

impl<T> Path<T> where T: ImageRepresentation {
    pub fn with_origin(mut self, offset: Vec2D) -> Self {
        self.segments = self.segments.iter().map(|segment| {
            match segment {
                Segment::CubicTo(point, handle_1, handle_2) => Segment::CubicTo(*point - offset, *handle_1 - offset, *handle_2 - offset),
                Segment::QuadraticTo(point, handle) => Segment::QuadraticTo(*point - offset, *handle - offset),
                Segment::MoveTo(point) => Segment::MoveTo(*point - offset),
                Segment::LineTo(point) => Segment::LineTo(*point - offset)
            }
        }).collect();
        self
    }
}

#[derive(Default)]
pub struct Builder {
    segments: Vec<Segment>,
}

impl Builder {
    pub fn new() -> Self {
        Builder::default()
    }
    pub fn line_to(mut self, to: Vec2D) -> Self {
        self.segments.push(Segment::LineTo(to));
        self
    }
    pub fn quadratic_to(mut self, to: Vec2D, handle: Vec2D) -> Self {
        self.segments.push(Segment::QuadraticTo(to, handle));
        self
    }
    pub fn bezier_to(mut self, to: Vec2D, handle_1: Vec2D, handle_2: Vec2D) -> Self {
        self.segments.push(Segment::CubicTo(to, handle_1, handle_2));
        self
    }
    pub fn done<T>(self) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        StyleHelper::new(self.segments)
    }
}

pub struct Primitive {}

impl Primitive {
    pub fn rectangle<T>(size: Vec2D) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        StyleHelper::new(vec![
            Segment::LineTo((size.x, 0.).into()),
            Segment::LineTo((size.x, size.y).into()),
            Segment::LineTo((0., size.y).into()),
        ])
    }
    pub fn rounded_rectangle<T>(size: Vec2D, radius: f64) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        StyleHelper::new(vec![
            Segment::MoveTo((radius, 0.).into()),
            Segment::LineTo((size.x - radius, 0.).into()),
            Segment::CubicTo(
                (size.x, radius).into(),
                (
                    size.x - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    0.,
                ).into(),
                (
                    size.x,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ).into(),
            ),
            Segment::LineTo((size.x, size.y - radius).into()),
            Segment::CubicTo(
                (size.x - radius, size.y).into(),
                (
                    size.x,
                    size.y - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ).into(),
                (
                    size.x - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    size.y,
                ).into(),
            ),
            Segment::LineTo((radius, size.y).into()),
            Segment::CubicTo(
                (0., size.y - radius).into(),
                (
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    size.y,
                ).into(),
                (
                    0.,
                    size.y - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ).into(),
            ),
            Segment::LineTo((0., radius).into()),
            Segment::CubicTo(
                (radius, 0.).into(),
                (0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)).into(),
                (radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.).into(),
            ),
        ])
    }
    pub fn square<T>(side_length: f64) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        Primitive::rectangle((side_length, side_length).into())
    }
    pub fn rounded_square<T>(side_length: f64, radius: f64) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        Primitive::rounded_rectangle((side_length, side_length).into(), radius)
    }
    pub fn circle<T>(radius: f64) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        StyleHelper::new(vec![
            Segment::MoveTo((radius, 0.).into()),
            Segment::CubicTo(
                (radius * 2., radius).into(),
                (radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.).into(),
                (
                    radius * 2.,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ).into(),
            ),
            Segment::CubicTo(
                (radius, radius * 2.).into(),
                (
                    radius * 2.,
                    radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ).into(),
                (
                    radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    radius * 2.,
                ).into(),
            ),
            Segment::CubicTo(
                (0., radius).into(),
                (
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    radius * 2.,
                ).into(),
                (0., radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)).into(),
            ),
            Segment::CubicTo(
                (radius, 0.).into(),
                (0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)).into(),
                (radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.).into(),
            ),
        ])
    }
    pub fn continuous_curvature_rectangle<T>(
        radii: Vec2D,
        k_factor: f64,
    ) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        StyleHelper::new(vec![
            Segment::MoveTo((radii.x, 0.).into()),
            Segment::CubicTo(
                (radii.x * 2., radii.y).into(),
                (radii.x * (1. + k_factor), 0.).into(),
                (radii.x * 2., radii.y * (1. - k_factor)).into(),
            ),
            Segment::CubicTo(
                (radii.x, radii.y * 2.).into(),
                (radii.x * 2., radii.y * (1. + k_factor)).into(),
                (radii.x * (1. + k_factor), radii.y * 2.).into(),
            ),
            Segment::CubicTo(
                (0., radii.y).into(),
                (radii.x * (1. - k_factor), radii.y * 2.).into(),
                (0., radii.y * (1. + k_factor)).into(),
            ),
            Segment::CubicTo(
                (radii.x, 0.).into(),
                (0., radii.y * (1. - k_factor)).into(),
                (radii.x * (1. - k_factor), 0.).into(),
            ),
        ])
    }
    pub fn continuous_curvature_square<T>(radius: f64, k_factor: f64) -> StyleHelper<T>
    where
        T: ImageRepresentation,
    {
        Primitive::continuous_curvature_rectangle((radius,radius).into(),k_factor)
    }
}

pub struct StyleHelper<T>
where
    T: ImageRepresentation,
{
    closed: bool,
    geometry: Vec<Segment>,
    fill: Option<Fill<T>>,
    stroke: Option<Stroke<T>>,
    shadow: Option<Shadow>,
}

impl<T> StyleHelper<T>
where
    T: ImageRepresentation,
{
    pub fn new(geometry: Vec<Segment>) -> Self {
        StyleHelper {
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
    pub fn fill(mut self, fill: Fill<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, stroke: Stroke<T>) -> Self
    where
        T: ImageRepresentation,
    {
        self.stroke = Some(stroke);
        self
    }
    pub fn shadow(mut self, shadow: Shadow) -> Self
    where
        T: ImageRepresentation,
    {
        self.shadow = Some(shadow);
        self
    }
    pub fn finalize(self) -> Path<T>
    where
        T: ImageRepresentation,
    {
        Path {
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
    stroke: Stroke<T>,
}

impl<T> StrokeBuilder<T>
where
    T: ImageRepresentation,
{
    pub fn new(content: Texture<T>, width: f32) -> Self {
        let mut builder = StrokeBuilder {
            stroke: Stroke::default(),
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
    pub fn finalize(self) -> Stroke<T> {
        self.stroke
    }
}
