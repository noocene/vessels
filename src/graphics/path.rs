use crate::graphics::{Color, ImageRepresentation, Rect, Vector2};

use crate::errors::Error;

use std::fmt;
use std::fmt::{Debug, Formatter};

const CUBIC_BEZIER_CIRCLE_APPROXIMATION_RATIO: f64 = 0.552_228_474;

/// A path segment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    /// A line to the given point.
    LineTo(Vector2),
    /// A movement of the pen to the given point.
    MoveTo(Vector2),
    /// A quadratic bezier curve to the given point with the given handle.
    QuadraticTo(Vector2, Vector2),
    /// A cubic bezier curve to the given point with the given handles.
    CubicTo(Vector2, Vector2, Vector2),
}

/// A gradient color stop.
#[derive(Clone, Copy, Debug)]
pub struct GradientStop {
    /// The offset, as a floating point value between zero and one, of the gradient stop.
    /// Zero represents the start of the gradient; one represents the end.
    pub offset: f64,
    /// The color of the stop.
    pub color: Color,
}

impl GradientStop {
    /// Creates a new gradient stop with the provided offset and color data.
    pub fn new(offset: f64, color: Color) -> Result<Self, Error> {
        if offset > 1.0 || offset < 0.0 {
            return Err(Error::color_stop());
        }

        Ok(GradientStop { offset, color })
    }
}

/// A linear gradient.
#[derive(Clone, Debug)]
pub struct LinearGradient {
    /// Associated color stops.
    pub stops: Vec<GradientStop>,
    /// The start point.
    pub start: Vector2,
    /// The end point.
    pub end: Vector2,
}

/// A drop shadow.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shadow {
    /// The color of the shadow.
    pub color: Color,
    /// The offset of the shadow.
    pub offset: Vector2,
    /// The blur radius, in fractional pixels, of the shadow.
    pub blur: f64,
    /// The spread radius, in fractional pixels, of the shadow.
    pub spread: f64,
}

impl Shadow {
    /// Creates a new shadow.
    pub fn new(color: Color) -> Self {
        Shadow {
            color,
            offset: Vector2::default(),
            blur: 0.,
            spread: 0.,
        }
    }
    /// Sets the blur radius.
    pub fn blur(mut self, amount: f64) -> Self {
        self.blur = amount;
        self
    }
    /// Sets the spread radius.
    pub fn spread(mut self, amount: f64) -> Self {
        self.spread = amount;
        self
    }
    /// Sets the offset.
    pub fn offset<T>(mut self, distance: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.offset = distance.into();
        self
    }
}

/// A radial gradient.
#[derive(Clone, Debug)]
pub struct RadialGradient {
    /// Associated color stops.
    pub stops: Vec<GradientStop>,
    /// The start point.
    pub start: Vector2,
    /// The radius at the start.
    pub start_radius: f64,
    /// The end point.
    pub end: Vector2,
    /// The radius at the end.
    pub end_radius: f64,
}

/// A texture used as the content for a stroke or fill.
#[derive(Clone)]
pub enum Texture {
    /// A solid color texture.
    Solid(Color),
    /// A linear gradient texture.
    LinearGradient(LinearGradient),
    /// A radial gradient texture.
    RadialGradient(RadialGradient),
    /// An image texture.
    Image(Box<dyn ImageRepresentation>),
}

impl From<Color> for Texture {
    fn from(color: Color) -> Texture {
        Texture::Solid(color)
    }
}

impl Debug for Texture {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Texture ( {} )",
            match self {
                Texture::Solid(color) => format!("Solid {:?}", color),
                Texture::LinearGradient(gradient) => format!("LinearGradient {:?}", gradient),
                Texture::RadialGradient(gradient) => format!("RadialGradient {:?}", gradient),
                Texture::Image(_) => "Image".to_owned(),
            }
        )
    }
}

impl From<Box<dyn ImageRepresentation>> for Texture {
    fn from(interaction: Box<dyn ImageRepresentation>) -> Self {
        Texture::Image(interaction)
    }
}

/// A stroke around the exterior of a path.
#[derive(Clone, Debug)]
pub struct Stroke {
    /// The texture of the stroke.
    pub content: Texture,
    /// The width of the stroke.
    pub width: f32,
    /// The style of the stroke ends.
    pub cap: StrokeCapType,
    /// The style of the joins between stroke segments.
    pub join: StrokeJoinType,
}

impl Default for Stroke {
    fn default() -> Self {
        Stroke {
            content: Color::black().into(),
            cap: StrokeCapType::Butt,
            join: StrokeJoinType::Miter,
            width: 1.,
        }
    }
}

/// Specifies the style of free stroke ends.
#[derive(Clone, Copy, Debug)]
pub enum StrokeCapType {
    /// Flat stroke caps.
    Butt,
    /// Rounded stroke caps.
    Round,
}

/// Specifies the style at the join of stroke segments.
#[derive(Clone, Copy, Debug)]
pub enum StrokeJoinType {
    /// A beveled stroke join.
    Bevel,
    /// A rounded stroke join.
    Round,
    /// A mitered stroke join.
    Miter,
}

/// A fill style within a closed path.
#[derive(Clone, Debug)]
pub struct Fill {
    /// The contents of the fill.
    pub content: Texture,
}

impl<T> From<T> for Fill
where
    T: Into<Texture>,
{
    fn from(interaction: T) -> Self {
        Fill {
            content: interaction.into(),
        }
    }
}

/// A vector graphics path in 2-dimensional euclidean space.
#[derive(Clone, Debug)]
pub struct Path {
    /// The segments comprising the path.
    pub segments: Vec<Segment>,
    /// The segments comprising the path's clipping mask.
    ///
    /// If this is empty the path is rendered in full.
    pub clip_segments: Vec<Segment>,
    /// The exterior stroke styling.
    pub stroke: Option<Stroke>,
    /// The internal fill.
    pub fill: Option<Fill>,
    /// The associated drop shadow.
    pub shadows: Vec<Shadow>,
    /// Whether the path is closed.
    pub closed: bool,
}

impl Path {
    /// Adjusts the origin of the path.
    pub fn with_offset<U>(mut self, offset: U) -> Self
    where
        U: Into<Vector2>,
    {
        let offset = offset.into();
        self.segments = self
            .segments
            .iter()
            .map(|segment| match segment {
                Segment::CubicTo(point, handle_1, handle_2) => {
                    Segment::CubicTo(*point + offset, *handle_1 + offset, *handle_2 + offset)
                }
                Segment::QuadraticTo(point, handle) => {
                    Segment::QuadraticTo(*point + offset, *handle + offset)
                }
                Segment::MoveTo(point) => Segment::MoveTo(*point + offset),
                Segment::LineTo(point) => Segment::LineTo(*point + offset),
            })
            .collect();
        self.clip_segments = self
            .clip_segments
            .iter()
            .map(|segment| match segment {
                Segment::CubicTo(point, handle_1, handle_2) => {
                    Segment::CubicTo(*point + offset, *handle_1 + offset, *handle_2 + offset)
                }
                Segment::QuadraticTo(point, handle) => {
                    Segment::QuadraticTo(*point + offset, *handle + offset)
                }
                Segment::MoveTo(point) => Segment::MoveTo(*point + offset),
                Segment::LineTo(point) => Segment::LineTo(*point + offset),
            })
            .collect();
        self
    }
    /// Computes an axis-aligned local coordinates bounding box of the path.
    pub fn bounds(&self) -> Rect {
        let mut top_left: Vector2 = (std::f64::INFINITY, std::f64::INFINITY).into();
        let mut bottom_right = Vector2::default();
        let mut update = |point: &Vector2| {
            if point.x < top_left.x {
                top_left.x = point.x;
            }
            if point.y < top_left.y {
                top_left.y = point.y;
            }
            if point.x > bottom_right.x {
                bottom_right.x = point.x;
            }
            if point.y > bottom_right.y {
                bottom_right.y = point.y;
            }
        };
        for segment in &self.segments {
            match segment {
                Segment::CubicTo(point, handle_1, handle_2) => {
                    update(point);
                    update(handle_1);
                    update(handle_2);
                }
                Segment::QuadraticTo(point, handle) => {
                    update(point);
                    update(handle);
                }
                Segment::MoveTo(point) => {
                    update(point);
                }
                Segment::LineTo(point) => {
                    update(point);
                }
            }
        }
        Rect::new(
            (top_left.x, top_left.y),
            (bottom_right.x - top_left.x, bottom_right.y - top_left.y),
        )
    }
}

/// Provides an interface for ergonomically building paths.
#[derive(Default, Debug)]
pub struct Builder {
    segments: Vec<Segment>,
}

impl Builder {
    /// Creates a new path.
    pub fn new() -> Self {
        Builder::default()
    }
    /// Draws a line to the specified point.
    pub fn line_to<T>(mut self, to: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.segments.push(Segment::LineTo(to.into()));
        self
    }
    /// Moves the pen to the specified point.
    pub fn move_to<T>(mut self, to: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.segments.push(Segment::MoveTo(to.into()));
        self
    }
    /// Draws a quadratic bezier curve to the specified point with the given handle.
    pub fn quadratic_to<T>(mut self, to: T, handle: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.segments
            .push(Segment::QuadraticTo(to.into(), handle.into()));
        self
    }
    /// Draws a cubic bezier curve to the specified point with the given handles.
    pub fn cubic_to<T>(mut self, to: T, handle_1: T, handle_2: T) -> Self
    where
        T: Into<Vector2>,
    {
        self.segments.push(Segment::CubicTo(
            to.into(),
            handle_1.into(),
            handle_2.into(),
        ));
        self
    }
    /// Finishes the path and returns a style builder containing the generated segments.
    pub fn done(self) -> StyleHelper {
        StyleHelper::new(self.segments)
    }
}

/// A helper for creating common vector graphics primitives.
#[derive(Clone, Copy, Debug)]
pub struct Primitive {}

impl Primitive {
    /// Creates a rectangle.
    pub fn rectangle<T>(size: T) -> StyleHelper
    where
        T: Into<Vector2>,
    {
        let size: Vector2 = size.into();
        Builder::new()
            .move_to((0., 0.))
            .line_to((size.x, 0.))
            .line_to((size.x, size.y))
            .line_to((0., size.y))
            .line_to((0., 0.))
            .done()
    }
    /// Creates a rounded rectangle.
    pub fn rounded_rectangle<T>(size: T, radius: f64) -> StyleHelper
    where
        T: Into<Vector2>,
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
    /// Creates a square.
    pub fn square(side_length: f64) -> StyleHelper {
        Primitive::rectangle((side_length, side_length))
    }
    /// Creates a rounded square.
    pub fn rounded_square(side_length: f64, radius: f64) -> StyleHelper {
        Primitive::rounded_rectangle((side_length, side_length), radius)
    }
    /// Creates a circle.
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
    /// Creates cubic-bezier approximation of a superellipse with the provided radii and k-factor.
    pub fn continuous_curvature_rectangle<T>(radii: T, k_factor: f64) -> StyleHelper
    where
        T: Into<Vector2>,
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
    /// Creates cubic-bezier approximation of a squircle with the provided radius and k-factor.
    pub fn continuous_curvature_square(radius: f64, k_factor: f64) -> StyleHelper {
        Primitive::continuous_curvature_rectangle((radius, radius), k_factor)
    }
}

/// Provides an ergonomic interface for building styled paths.
#[derive(Debug)]
pub struct StyleHelper {
    closed: bool,
    geometry: Vec<Segment>,
    clip_geometry: Vec<Segment>,
    fill: Option<Fill>,
    stroke: Option<Stroke>,
    shadows: Vec<Shadow>,
}

impl Into<Vec<Segment>> for StyleHelper {
    fn into(self) -> Vec<Segment> {
        self.geometry
    }
}

impl Into<Vec<Segment>> for Path {
    fn into(self) -> Vec<Segment> {
        self.segments
    }
}

impl StyleHelper {
    /// Creates a new [StyleHelper].
    pub fn new(geometry: Vec<Segment>) -> Self {
        StyleHelper {
            closed: false,
            geometry,
            clip_geometry: vec![],
            fill: None,
            shadows: vec![],
            stroke: None,
        }
    }
    /// Marks the path as closed.
    pub fn close(mut self) -> Self {
        self.closed = true;
        self
    }
    /// Applies the provided clipping mask.
    ///
    /// This clipping mask applies to shadows in addition to content, to draw shadows outside a clipped entity a separate path is necessary.
    pub fn clip<T: Into<Vec<Segment>>>(mut self, clip_path: T) -> Self {
        let clip_path = clip_path.into();
        self.clip_geometry = clip_path;
        self
    }
    /// Fills the path with the provided texture.
    pub fn fill(mut self, fill: Fill) -> Self {
        self.fill = Some(fill);
        self
    }
    /// Strokes the path using the provided style information.
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
    /// Shadows the path using the provided style information.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadows.push(shadow);
        self
    }
    /// Finalizes the styling and returns a styled [Path].
    pub fn finalize(self) -> Path {
        Path {
            closed: self.closed,
            segments: self.geometry,
            fill: self.fill,
            shadows: self.shadows,
            stroke: self.stroke,
            clip_segments: self.clip_geometry,
        }
    }
}

/// Provides an ergonomic interface for building stroke styling.
#[derive(Debug)]
pub struct StrokeBuilder {
    stroke: Stroke,
}

impl StrokeBuilder {
    /// Creates a new [StrokeBuilder] with the provided stroke contents and stroke width.
    pub fn new(content: Texture, width: f32) -> Self {
        let mut builder = StrokeBuilder {
            stroke: Stroke::default(),
        };
        builder.stroke.content = content;
        builder.stroke.width = width;
        builder
    }
    /// Sets the stroke cap type to rounded caps.
    pub fn cap_round(mut self) -> Self {
        self.stroke.cap = StrokeCapType::Round;
        self
    }
    /// Sets the stroke join type to beveled joins.
    pub fn join_bevel(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Bevel;
        self
    }
    /// Sets the stroke join type to rounded joins.
    pub fn join_round(mut self) -> Self {
        self.stroke.join = StrokeJoinType::Round;
        self
    }
    /// Finalizes the style and returns a completed [Stroke].
    pub fn finalize(self) -> Stroke {
        self.stroke
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, GradientStop};

    #[test]
    fn gradient_stop_fail() {
        assert!(GradientStop::new(5.0, Color::white()).is_err());
        assert!(GradientStop::new(-5.0, Color::white()).is_err());
    }
}
