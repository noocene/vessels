use crate::graphics_2d::*;

use crate::errors::Error;

const CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO: f64 = 0.552_228_474;

#[derive(Clone, Copy)]
pub enum Segment {
    LineTo(Vector),
    MoveTo(Vector),
    QuadraticTo(Vector, Vector),
    CubicTo(Vector, Vector, Vector),
}

#[derive(Clone, Copy)]
pub struct GradientStop {
    pub offset: f64,
    pub color: RGBA8,
}

impl GradientStop {
    pub fn new(offset: f64, color: RGBA8) -> Result<Self, Error> {
        if offset > 1.0 || offset < 0.0 {
            return Err(Error::color_stop());
        }

        Ok(GradientStop { offset, color })
    }
}

#[derive(Clone)]
pub struct LinearGradient {
    pub stops: Vec<GradientStop>,
    pub start: Vector,
    pub end: Vector,
}

#[derive(Clone)]
pub struct Shadow {
    pub color: RGBA8,
    pub offset: Vector,
    pub blur: f64,
}

impl Shadow {
    pub fn new(color: RGBA8) -> Self {
        Shadow {
            color,
            offset: Vector::default(),
            blur: 0.,
        }
    }
    pub fn blur(mut self, amount: f64) -> Self {
        self.blur = amount;
        self
    }
    pub fn offset<T>(mut self, distance: T) -> Self
    where
        T: Into<Vector>,
    {
        self.offset = distance.into();
        self
    }
}

#[derive(Clone)]
pub struct RadialGradient {
    pub stops: Vec<GradientStop>,
    pub start: Vector,
    pub start_radius: f64,
    pub end: Vector,
    pub end_radius: f64,
}

#[derive(Clone)]
pub enum Texture {
    Solid(RGBA8),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
    Image(Box<dyn ImageRepresentation>),
}

impl<T: 'static> From<T> for Texture
where
    T: ImageRepresentation,
{
    fn from(input: T) -> Self {
        Texture::Image(Box::new(input))
    }
}

#[derive(Clone)]
pub struct Stroke {
    pub content: Texture,
    pub width: f32,
    pub cap: StrokeCapType,
    pub join: StrokeJoinType,
}

impl Default for Stroke {
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
pub struct Fill {
    pub content: Texture,
}

impl<T> From<T> for Fill
where
    T: Into<Texture>,
{
    fn from(input: T) -> Self {
        Fill {
            content: input.into(),
        }
    }
}

#[derive(Clone)]
pub struct Path {
    pub orientation: Transform,
    pub segments: Vec<Segment>,
    pub stroke: Option<Stroke>,
    pub fill: Option<Fill>,
    pub shadow: Option<Shadow>,
    pub closed: bool,
}

impl Path {
    pub fn with_origin<U>(mut self, offset: U) -> Self
    where
        U: Into<Vector>,
    {
        let offset = offset.into();
        self.segments = self
            .segments
            .iter()
            .map(|segment| match segment {
                Segment::CubicTo(point, handle_1, handle_2) => {
                    Segment::CubicTo(*point - offset, *handle_1 - offset, *handle_2 - offset)
                }
                Segment::QuadraticTo(point, handle) => {
                    Segment::QuadraticTo(*point - offset, *handle - offset)
                }
                Segment::MoveTo(point) => Segment::MoveTo(*point - offset),
                Segment::LineTo(point) => Segment::LineTo(*point - offset),
            })
            .collect();
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
    pub fn line_to<T>(mut self, to: T) -> Self
    where
        T: Into<Vector>,
    {
        self.segments.push(Segment::LineTo(to.into()));
        self
    }
    pub fn move_to<T>(mut self, to: T) -> Self
    where
        T: Into<Vector>,
    {
        self.segments.push(Segment::MoveTo(to.into()));
        self
    }
    pub fn quadratic_to<T>(mut self, to: T, handle: T) -> Self
    where
        T: Into<Vector>,
    {
        self.segments
            .push(Segment::QuadraticTo(to.into(), handle.into()));
        self
    }
    pub fn cubic_to<T>(mut self, to: T, handle_1: T, handle_2: T) -> Self
    where
        T: Into<Vector>,
    {
        self.segments.push(Segment::CubicTo(
            to.into(),
            handle_1.into(),
            handle_2.into(),
        ));
        self
    }
    pub fn done(self) -> StyleHelper {
        StyleHelper::new(self.segments)
    }
}

pub struct Primitive {}

impl Primitive {
    pub fn rectangle<T>(size: T) -> StyleHelper
    where
        T: Into<Vector>,
    {
        let size: Vector = size.into();
        Builder::new()
            .move_to((0., 0.))
            .line_to((size.x, 0.))
            .line_to((size.x, size.y))
            .line_to((0., size.y))
            .line_to((0., 0.))
            .done()
    }
    pub fn rounded_rectangle<T>(size: T, radius: f64) -> StyleHelper
    where
        T: Into<Vector>,
    {
        let size = size.into();
        Builder::new()
            .move_to((radius, 0.))
            .line_to((size.x - radius, 0.))
            .cubic_to(
                (size.x, radius),
                (
                    size.x - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    0.,
                ),
                (
                    size.x,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            )
            .line_to((size.x, size.y - radius))
            .cubic_to(
                (size.x - radius, size.y),
                (
                    size.x,
                    size.y - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
                (
                    size.x - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    size.y,
                ),
            )
            .line_to((radius, size.y))
            .cubic_to(
                (0., size.y - radius),
                (
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    size.y,
                ),
                (
                    0.,
                    size.y - radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            )
            .line_to((0., radius))
            .cubic_to(
                (radius, 0.),
                (0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)),
                (radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.),
            )
            .done()
    }
    pub fn square(side_length: f64) -> StyleHelper {
        Primitive::rectangle((side_length, side_length))
    }
    pub fn rounded_square(side_length: f64, radius: f64) -> StyleHelper {
        Primitive::rounded_rectangle((side_length, side_length), radius)
    }
    pub fn circle(radius: f64) -> StyleHelper {
        Builder::new()
            .move_to((radius, 0.))
            .cubic_to(
                (radius * 2., radius),
                (radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.),
                (
                    radius * 2.,
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
            )
            .cubic_to(
                (radius, radius * 2.),
                (
                    radius * 2.,
                    radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                ),
                (
                    radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    radius * 2.,
                ),
            )
            .cubic_to(
                (0., radius),
                (
                    radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO),
                    radius * 2.,
                ),
                (0., radius * (1. + CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)),
            )
            .cubic_to(
                (radius, 0.),
                (0., radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO)),
                (radius * (1. - CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO), 0.),
            )
            .done()
    }
    pub fn continuous_curvature_rectangle<T>(radii: T, k_factor: f64) -> StyleHelper
    where
        T: Into<Vector>,
    {
        let radii = radii.into();
        Builder::new()
            .move_to((radii.x, 0.))
            .cubic_to(
                (radii.x * 2., radii.y),
                (radii.x * (1. + k_factor), 0.),
                (radii.x * 2., radii.y * (1. - k_factor)),
            )
            .cubic_to(
                (radii.x, radii.y * 2.),
                (radii.x * 2., radii.y * (1. + k_factor)),
                (radii.x * (1. + k_factor), radii.y * 2.),
            )
            .cubic_to(
                (0., radii.y),
                (radii.x * (1. - k_factor), radii.y * 2.),
                (0., radii.y * (1. + k_factor)),
            )
            .cubic_to(
                (radii.x, 0.),
                (0., radii.y * (1. - k_factor)),
                (radii.x * (1. - k_factor), 0.),
            )
            .done()
    }
    pub fn continuous_curvature_square(radius: f64, k_factor: f64) -> StyleHelper {
        Primitive::continuous_curvature_rectangle((radius, radius), k_factor)
    }
}

pub struct StyleHelper {
    closed: bool,
    geometry: Vec<Segment>,
    fill: Option<Fill>,
    stroke: Option<Stroke>,
    shadow: Option<Shadow>,
}

impl StyleHelper {
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
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = Some(shadow);
        self
    }
    pub fn finalize(self) -> Path {
        Path {
            closed: self.closed,
            segments: self.geometry,
            orientation: Transform::default(),
            fill: self.fill,
            shadow: self.shadow,
            stroke: self.stroke,
        }
    }
}

pub struct StrokeBuilder {
    stroke: Stroke,
}

impl StrokeBuilder {
    pub fn new(content: Texture, width: f32) -> Self {
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
    pub fn finalize(self) -> Stroke {
        self.stroke
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_stop_fail() {
        assert!(GradientStop::new(5.0, RGBA8::white()).is_err());
        assert!(GradientStop::new(-5.0, RGBA8::white()).is_err());
    }
}
