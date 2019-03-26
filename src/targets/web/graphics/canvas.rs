use crate::graphics_2d::{
    Color, ContextGraphics, ContextualGraphics, Frame, Graphics, Image, ImageRepresentation,
    Object, Rasterizable, Rasterizer, Rect, Texture2D, Ticker, Transform, Vector,
};
use crate::input::Context;
use crate::path::{Path, Segment, StrokeCapType, StrokeJoinType, Texture};
use crate::targets::web;
use crate::text::{Align, Font, Text, Weight, Wrap};
use crate::util::ObserverCell;

use stdweb::traits::{IChildNode, IElement, IEvent, IEventTarget, IHtmlElement, INode};
use stdweb::unstable::TryInto;
use stdweb::web::{
    document, window, CanvasPattern, CanvasRenderingContext2d, FillRule, LineCap, LineJoin,
    TextAlign, TextBaseline,
};

use stdweb::web::event::{ContextMenuEvent, ResizeEvent};

use stdweb::web::html_element::CanvasElement;

use std::sync::{Arc, RwLock};

use std::ops::Deref;

use std::any::Any;

type CanvasImage = CanvasElement;

impl ImageRepresentation for CanvasImage {
    fn get_size(&self) -> Vector {
        let dpr = window().device_pixel_ratio();
        (
            f64::from(self.width()) / dpr,
            f64::from(self.height()) / dpr,
        )
            .into()
    }
    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }
    fn as_texture(&self) -> Image<Color, Texture2D> {
        Image {
            pixels: vec![],
            format: Texture2D {
                height: 0,
                width: 0,
            },
        }
    }
    fn from_texture(texture: Image<Color, Texture2D>) -> CanvasImage {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(texture.format.width);
        canvas.set_height(texture.format.height);
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let image = context
            .create_image_data(
                f64::from(texture.format.width),
                f64::from(texture.format.height),
            )
            .unwrap();
        context.put_image_data(image, 0., 0.).unwrap();
        canvas
    }
}

struct CanvasObjectState {
    orientation: Transform,
    content: Rasterizable,
}

#[derive(Clone)]
struct CanvasObject {
    state: Arc<RwLock<CanvasObjectState>>,
}

impl CanvasObject {
    fn new(content: Rasterizable, orientation: Transform) -> CanvasObject {
        CanvasObject {
            state: Arc::new(RwLock::new(CanvasObjectState {
                orientation,
                content,
            })),
        }
    }
}

impl Object for CanvasObject {
    fn get_transform(&self) -> Transform {
        self.state.read().unwrap().orientation
    }
    fn apply_transform(&mut self, transform: Transform) {
        self.state.write().unwrap().orientation.transform(transform);
    }
    fn set_transform(&mut self, transform: Transform) {
        self.state.write().unwrap().orientation = transform;
    }
    fn update(&mut self, input: Rasterizable) {
        self.state.write().unwrap().content = input;
    }
}

struct CanvasFrameState {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
    contents: Vec<CanvasObject>,
    pixel_ratio: f64,
    viewport: Rect,
    size: Vector,
}

impl Drop for CanvasFrameState {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}

struct CanvasFrame {
    state: Arc<RwLock<CanvasFrameState>>,
}

impl CanvasFrame {
    fn new() -> CanvasFrame {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        CanvasFrame {
            state: Arc::new(RwLock::new(CanvasFrameState {
                canvas,
                pixel_ratio: window().device_pixel_ratio(),
                context,
                contents: vec![],
                size: Vector::default(),
                viewport: Rect::default(),
            })),
        }
    }
    fn show(&self) {
        let state = self.state.read().unwrap();
        state.canvas.add_event_listener(|event: ContextMenuEvent| {
            event.prevent_default();
            event.stop_propagation();
        });
        document().body().unwrap().append_child(&state.canvas);
    }
    fn draw_path(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        state.context.restore();
        state.context.save();
        state.context.transform(
            matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
        );
        state.context.begin_path();
        match &entity.shadow {
            Some(shadow) => {
                state.context.set_shadow_blur(shadow.blur);
                state
                    .context
                    .set_shadow_color(&shadow.color.to_rgba_color());
                state.context.set_shadow_offset_x(shadow.offset.x);
                state.context.set_shadow_offset_y(shadow.offset.y);
            }
            None => {
                state.context.set_shadow_color("rgba(0,0,0,0)");
            }
        }
        let segments = entity.segments.iter();
        state.context.move_to(0., 0.);
        segments.for_each(|segment| match segment {
            Segment::LineTo(point) => {
                state.context.line_to(point.x, point.y);
            }
            Segment::MoveTo(point) => {
                state.context.move_to(point.x, point.y);
            }
            Segment::CubicTo(point, handle_1, handle_2) => {
                state.context.bezier_curve_to(
                    handle_1.x, handle_1.y, handle_2.x, handle_2.y, point.x, point.y,
                );
            }
            Segment::QuadraticTo(point, handle) => {
                state
                    .context
                    .quadratic_curve_to(handle.x, handle.y, point.x, point.y);
            }
        });
        if entity.closed {
            state.context.close_path();
        }
        match &entity.stroke {
            Some(stroke) => {
                state.context.set_line_cap(match &stroke.cap {
                    StrokeCapType::Butt => LineCap::Butt,
                    StrokeCapType::Round => LineCap::Round,
                });
                state.context.set_line_join(match &stroke.join {
                    StrokeJoinType::Miter => LineJoin::Miter,
                    StrokeJoinType::Round => LineJoin::Round,
                    StrokeJoinType::Bevel => LineJoin::Bevel,
                });
                match &stroke.content {
                    Texture::Solid(color) => {
                        state.context.set_stroke_style_color(&color.to_rgba_color());
                    }
                    Texture::LinearGradient(gradient) => {
                        let canvas_gradient = state.context.create_linear_gradient(
                            gradient.start.x,
                            gradient.start.y,
                            gradient.end.x,
                            gradient.end.y,
                        );
                        gradient.stops.iter().for_each(|stop| {
                            canvas_gradient
                                .add_color_stop(stop.offset, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_stroke_style_gradient(&canvas_gradient);
                    }
                    Texture::Image(image) => {
                        let pattern: CanvasPattern = match image.as_any().downcast::<CanvasImage>() {
                            Ok(as_image) => js! {
                                return @{&state.context}.createPattern(@{as_image.deref()}, "no-repeat");
                            }.try_into().unwrap(),
                            Err(_) => {
                                let as_image = CanvasImage::from_texture(image.box_clone().as_texture());
                                return js! {
                                    return @{&state.context}.createPattern(@{as_image}, "no-repeat");
                                }.try_into().unwrap();
                            }
                        };
                        state
                            .context
                            .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio);
                        state.context.set_stroke_style_pattern(&pattern);
                    }
                    Texture::RadialGradient(gradient) => {
                        let canvas_gradient = state
                            .context
                            .create_radial_gradient(
                                gradient.start.x,
                                gradient.start.y,
                                gradient.start_radius,
                                gradient.end.x,
                                gradient.end.y,
                                gradient.end_radius,
                            )
                            .unwrap();
                        gradient.stops.iter().for_each(|stop| {
                            canvas_gradient
                                .add_color_stop(stop.offset, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_stroke_style_gradient(&canvas_gradient);
                    }
                }
                state.context.set_line_width(f64::from(stroke.width));
                state.context.stroke();
                if let Texture::Image(_image) = &stroke.content {
                    state.context.scale(state.pixel_ratio, state.pixel_ratio);
                }
            }
            None => {}
        }
        match &entity.fill {
            Some(fill) => {
                match &fill.content {
                    Texture::Solid(color) => {
                        state.context.set_fill_style_color(&color.to_rgba_color());
                    }
                    Texture::Image(image) => {
                        let pattern: CanvasPattern = match image.as_any().downcast::<CanvasImage>() {
                            Ok(as_image) => js! {
                                return @{&state.context}.createPattern(@{as_image.deref()}, "no-repeat");
                            }.try_into().unwrap(),
                            Err(_) => {
                                let as_image = CanvasImage::from_texture(image.box_clone().as_texture());
                                return js! {
                                    return @{&state.context}.createPattern(@{as_image}, "no-repeat");
                                }.try_into().unwrap();
                            }
                        };
                        state
                            .context
                            .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio);
                        state.context.set_fill_style_pattern(&pattern);
                    }
                    Texture::LinearGradient(gradient) => {
                        let canvas_gradient = state.context.create_linear_gradient(
                            gradient.start.x,
                            gradient.start.y,
                            gradient.end.x,
                            gradient.end.y,
                        );
                        gradient.stops.iter().for_each(|stop| {
                            canvas_gradient
                                .add_color_stop(stop.offset, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_fill_style_gradient(&canvas_gradient);
                    }
                    Texture::RadialGradient(gradient) => {
                        let canvas_gradient = state
                            .context
                            .create_radial_gradient(
                                gradient.start.x,
                                gradient.start.y,
                                gradient.start_radius,
                                gradient.end.x,
                                gradient.end.y,
                                gradient.end_radius,
                            )
                            .unwrap();
                        gradient.stops.iter().for_each(|stop| {
                            canvas_gradient
                                .add_color_stop(stop.offset, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_fill_style_gradient(&canvas_gradient);
                    }
                }
                state.context.fill(FillRule::NonZero);
                if let Texture::Image(_image) = &fill.content {
                    state.context.scale(state.pixel_ratio, state.pixel_ratio);
                }
            }
            None => {}
        }
    }
    fn update_text_style(&self, input: &Text) {
        let state = self.state.read().unwrap();
        state.context.set_font((match input.font {
                Font::SystemFont => {
                    format!(r#"{} {} {}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol""#, if input.italic { "italic " } else { "" }, match input.weight {
                        Weight::Normal => "400",
                        Weight::Bold => "500",
                        Weight::Heavy => "700",
                        Weight::Thin => "200",
                        Weight::Light => "200",
                        Weight::Hairline => "100"
                    }, input.size)
                }
            }).as_str());
        state.context.set_text_align(match input.align {
            Align::Center => TextAlign::Center,
            Align::End => TextAlign::End,
            Align::Start => TextAlign::Start,
        });
        state.context.set_text_baseline(TextBaseline::Hanging);
        state
            .context
            .set_fill_style_color(&input.color.to_rgba_color());
    }
    fn draw_text(&self, matrix: [f64; 6], input: &Text) {
        let state = self.state.read().unwrap();
        state.context.restore();
        state.context.save();
        state.context.transform(
            matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
        );
        let mut lines: Vec<String> = input
            .content
            .split('\n')
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        self.update_text_style(&input);
        if input.max_width.is_some() {
            lines = self.wrap_text(&input);
        }
        for (index, line) in lines.iter().enumerate() {
            state.context.fill_text(
                line,
                0.,
                (u32::from(input.line_height) * index as u32).into(),
                None,
            );
        }
    }
    fn draw(&self) {
        let state = self.state.read().unwrap();
        let viewport = state.viewport;
        let size = state.size;
        state.context.set_transform(
            (size.x / viewport.size.x) * state.pixel_ratio,
            0.,
            0.,
            (size.y / viewport.size.y) * state.pixel_ratio,
            -viewport.position.x * state.pixel_ratio,
            -viewport.position.y * state.pixel_ratio,
        );
        state.context.clear_rect(
            viewport.position.x,
            viewport.position.y,
            viewport.size.x,
            viewport.size.y,
        );
        state.context.save();
        state.contents.iter().for_each(|object| {
            let object = object.state.read().unwrap();
            let matrix = object.orientation.to_matrix();
            match &object.content {
                Rasterizable::Path(path) => self.draw_path(matrix, &path),
                Rasterizable::Text(input) => self.draw_text(matrix, &input),
            };
        });
    }
    fn element(&self) -> CanvasElement {
        let state = self.state.read().unwrap();
        state.canvas.clone()
    }
    fn wrap_text(&self, input: &Text) -> Vec<String> {
        let state = self.state.read().unwrap();
        let mut lines: Vec<String> = input
            .content
            .split('\n')
            .map(std::borrow::ToOwned::to_owned)
            .collect();
        match input.wrap {
            Wrap::Normal => {
                let mut test_string = "".to_owned();
                lines.reverse();
                let mut wrapped_lines: Vec<String> = vec![];
                loop {
                    let line = lines.pop();
                    match line {
                        None => {
                            break;
                        }
                        Some(line) => {
                            let words = line.split(' ').collect::<Vec<&str>>();
                            for (index, word) in words.iter().cloned().enumerate() {
                                if state
                                    .context
                                    .measure_text(&(test_string.clone() + word))
                                    .unwrap()
                                    .get_width()
                                    <= f64::from(input.max_width.unwrap())
                                {
                                    test_string += &format!(" {}", word);
                                } else {
                                    test_string = test_string.trim().to_owned();
                                    wrapped_lines.push(test_string);
                                    lines.push(
                                        words
                                            .iter()
                                            .cloned()
                                            .skip(index)
                                            .collect::<Vec<&str>>()
                                            .join(" "),
                                    );
                                    test_string = "".to_owned();
                                    break;
                                }
                            }
                            if test_string != "" {
                                wrapped_lines.push(test_string.clone().trim().to_owned());
                            }
                        }
                    }
                }
                wrapped_lines
            }
            _ => lines,
        }
    }
}

impl Clone for CanvasFrame {
    fn clone(&self) -> Self {
        CanvasFrame {
            state: self.state.clone(),
        }
    }
}

impl Frame for CanvasFrame {
    type Object = CanvasObject;
    type Image = CanvasImage;
    fn add<T, U>(&mut self, rasterizable: T, orientation: U) -> Box<dyn Object>
    where
        T: Into<Rasterizable>,
        U: Into<Transform>,
    {
        let object = CanvasObject::new(rasterizable.into(), orientation.into());
        let mut state = self.state.write().unwrap();
        state.contents.push(object.clone());
        Box::new(object)
    }
    fn set_viewport(&self, viewport: Rect) {
        let mut state = self.state.write().unwrap();
        state.viewport = viewport;
    }
    fn resize<T>(&self, size: T)
    where
        T: Into<Vector>,
    {
        let mut state = self.state.write().unwrap();
        let size = size.into();
        state.size = size;
        state.canvas.set_height((size.y * state.pixel_ratio) as u32);
        state.canvas.set_width((size.x * state.pixel_ratio) as u32);
    }
    fn get_size(&self) -> Vector {
        let state = self.state.read().unwrap();
        state.size
    }
    fn to_image(&self) -> Box<CanvasImage> {
        let state = self.state.read().unwrap();
        self.draw();
        Box::new(state.canvas.clone())
    }
    fn measure(&self, input: Text) -> Vector {
        self.update_text_style(&input);
        let state = self.state.read().unwrap();
        if input.max_width.is_some() {
            let lines = self.wrap_text(&input);
            (
                f64::from(input.max_width.unwrap()),
                (f64::from((lines.len() - 1).max(0) as u32) * f64::from(input.line_height))
                    + f64::from(input.size),
            )
                .into()
        } else {
            (
                state
                    .context
                    .measure_text(&input.content)
                    .unwrap()
                    .get_width(),
                f64::from(input.size),
            )
                .into()
        }
    }
}

struct Canvas {
    state: Arc<RwLock<CanvasState>>,
}

struct CanvasState {
    root_frame: Option<CanvasFrame>,
    size: ObserverCell<Vector>,
    tick_handlers: Vec<Box<dyn FnMut(f64) + Send + Sync>>,
}

impl Rasterizer for Canvas {
    type Image = CanvasImage;
    fn rasterize<T>(&self, input: T, size: Vector) -> Box<dyn ImageRepresentation>
    where
        T: Into<Rasterizable>,
    {
        let input: Rasterizable = input.into();
        let mut frame = CanvasFrame::new();
        frame.resize(size);
        frame.set_viewport(Rect::new(Vector::default(), size));
        frame.add(input, Vector::from((0., 0.)));
        frame.draw();
        Box::new(frame.element())
    }
}

impl Context for Canvas {
    type Mouse = web::input::Mouse;
    type Keyboard = web::input::Keyboard;
    fn mouse(&self) -> Self::Mouse {
        web::input::Mouse::new()
    }
    fn keyboard(&self) -> Self::Keyboard {
        web::input::Keyboard::new()
    }
}

impl Ticker for Canvas {
    fn bind<F>(&mut self, handler: F)
    where
        F: FnMut(f64) + 'static + Send + Sync,
    {
        self.state
            .write()
            .unwrap()
            .tick_handlers
            .push(Box::new(handler));
    }
}

impl ContextGraphics for Canvas {
    fn run(self) {
        let state = self.state.read().unwrap();
        state.root_frame.as_ref().unwrap().show();
        let cloned = self.clone();
        window().request_animation_frame(move |start_time| {
            cloned.animate(start_time, start_time);
        });
    }
}

impl ContextualGraphics for Canvas {
    type Context = Canvas;
    fn start(self, root: CanvasFrame) -> Self::Context {
        {
            let mut state = self.state.write().unwrap();
            state.root_frame = Some(root);
        }
        self
    }
}

impl Graphics for Canvas {
    type Frame = CanvasFrame;
    fn frame(&self) -> CanvasFrame {
        CanvasFrame::new()
    }
}

impl Canvas {
    fn animate(&self, start_time: f64, last_start_time: f64) {
        let mut state = self.state.write().unwrap();
        state
            .tick_handlers
            .iter_mut()
            .for_each(|handler| (handler)(start_time - last_start_time));
        match &state.root_frame {
            Some(frame) => {
                if state.size.is_dirty() {
                    let size = state.size.get();
                    frame.resize(size);
                    frame.set_viewport(Rect::new((0., 0.), size));
                }
                frame.draw();
            }
            None => {}
        }
        let cloned = self.clone();
        window().request_animation_frame(move |new_start_time| {
            cloned.animate(new_start_time, start_time);
        });
    }
}

impl Clone for Canvas {
    fn clone(&self) -> Canvas {
        Canvas {
            state: self.state.clone(),
        }
    }
}

pub(crate) fn new() -> impl ContextualGraphics {
    document()
        .head()
        .unwrap()
        .append_html(
            r#"
<style>
body, html, canvas {
    height: 100%;
}
body {
    margin: 0;
    overflow: hidden;
}
canvas {
    width: 100%;
}
</style>
            "#,
        )
        .unwrap();

    let body = document().body().unwrap();

    let gfx = Canvas {
        state: Arc::new(RwLock::new(CanvasState {
            size: ObserverCell::new(
                (body.offset_width().into(), body.offset_height().into()).into(),
            ),
            root_frame: None,
            tick_handlers: vec![],
        })),
    };

    let gfx_resize = gfx.clone();

    window().add_event_listener(move |_: ResizeEvent| {
        let state = gfx_resize.state.read().unwrap();
        let body = document().body().unwrap();
        state
            .size
            .set((body.offset_width().into(), body.offset_height().into()).into());
    });

    gfx
}
