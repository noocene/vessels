use crate::graphics_2d::*;
use crate::input::*;
use crate::graphics_2d::text::*;
use crate::util::ObserverCell;
use crate::graphics_2d::path::*;
use crate::targets::web;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document, window, CanvasPattern, CanvasRenderingContext2d, FillRule, LineCap, LineJoin, TextAlign, TextBaseline,
};

use stdweb::web::event::{ResizeEvent, ContextMenuEvent};

use stdweb::web::html_element::CanvasElement;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::borrow::Cow;

use std::slice::Iter;

use std::ops::Deref;

type CanvasImage = CanvasElement;

impl ImageRepresentation for CanvasImage {
    fn get_size(&self) -> Vector {
        let dpr = window().device_pixel_ratio();
        (f64::from(self.width()) / dpr, f64::from(self.height()) / dpr).into()
    }
}

impl From<Image<RGBA8, Texture2D>> for CanvasImage {
    fn from(input: Image<RGBA8, Texture2D>) -> CanvasImage {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(input.format.width);
        canvas.set_height(input.format.height);
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let image = context
            .create_image_data(f64::from(input.format.width), f64::from(input.format.height))
            .unwrap();
        context.put_image_data(image, 0., 0.).unwrap();
        canvas
    }
}

impl Into<Image<RGBA8, Texture2D>> for CanvasImage {
    fn into(self) -> Image<RGBA8, Texture2D> {
        Image {
            pixels: vec![],
            format: Texture2D {
                height: 0,
                width: 0,
            },
        }
    }
}

struct CanvasFrame {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
    contents: Vec<Object<CanvasImage>>,
    pixel_ratio: f64,
    viewport: Cell<Rect>,
    size: Cell<Vector>,
}

impl Drop for CanvasFrame {
    fn drop(&mut self) {
        self.canvas.remove();
    }
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
            canvas,
            pixel_ratio: window().device_pixel_ratio(),
            context,
            contents: vec![],
            size: Cell::from(Vector::default()),
            viewport: Cell::from(Rect {
                size: Vector::default(),
                position: (0., 0.).into(),
            }),
        }
    }
    fn show(&self) {
        self.canvas.add_event_listener(|event: ContextMenuEvent| {
            event.prevent_default();
            event.stop_propagation();
        });
        document().body().unwrap().append_child(&self.canvas);
    }
    fn draw(&self) {
        let viewport = self.viewport.get();
        let size = self.size.get();
        self.context.set_transform(
            (size.x / viewport.size.x) * self.pixel_ratio,
            0.,
            0.,
            (size.y / viewport.size.y) * self.pixel_ratio,
            -viewport.position.x * self.pixel_ratio,
            -viewport.position.y * self.pixel_ratio,
        );
        self.context.clear_rect(
            viewport.position.x,
            viewport.position.y,
            viewport.size.x,
            viewport.size.y,
        );
        self.context.save();
        self.contents.iter().for_each(|object| {
            let draw = |orientation: Transform, content: Iter<Path<CanvasImage>>| {
                let matrix = orientation.to_matrix();
                content.for_each(|entity| {
                    self.context.restore();
                    self.context.save();
                    self.context.transform(matrix[0],matrix[1],matrix[2],matrix[3],matrix[4],matrix[5]);
                    let matrix = entity.orientation.to_matrix();
                    self.context.transform(matrix[0],matrix[1],matrix[2],matrix[3],matrix[4],matrix[5]);
                    self.context.begin_path();
                    match &entity.shadow {
                        Some(shadow) => {
                            self.context.set_shadow_blur(shadow.blur);
                            self.context.set_shadow_color(&shadow.color.to_rgba_color());
                            self.context.set_shadow_offset_x(shadow.offset.x);
                            self.context.set_shadow_offset_y(shadow.offset.y);
                        }
                        None => {
                            self.context.set_shadow_color("rgba(0,0,0,0)");
                        }
                    }
                    let segments = entity.segments.iter();
                    self.context.move_to(0., 0.);
                    segments.for_each(|segment| {
                        match segment {
                            Segment::LineTo(point) => {
                                self.context.line_to(
                                    point.x, point.y
                                );
                            },
                            Segment::MoveTo(point) => {
                                self.context.move_to(
                                   point.x, point.y
                                );
                            },
                            Segment::CubicTo(point, handle_1, handle_2) => {
                                self.context.bezier_curve_to(
                                    handle_1.x, handle_1.y, handle_2.x, handle_2.y, point.x, point.y 
                                );
                            }
                            Segment::QuadraticTo(point, handle) => {
                                self.context.quadratic_curve_to(
                                    handle.x, handle.y, point.x, point.y 
                                );
                            }
                        }
                    });
                    if entity.closed {
                        self.context.close_path();
                    }
                    match &entity.stroke {
                        Some(stroke) => {
                            self.context.set_line_cap(match &stroke.cap {
                                StrokeCapType::Butt => LineCap::Butt,
                                StrokeCapType::Round => LineCap::Round,
                            });
                            self.context.set_line_join(match &stroke.join {
                                StrokeJoinType::Miter => LineJoin::Miter,
                                StrokeJoinType::Round => LineJoin::Round,
                                StrokeJoinType::Bevel => LineJoin::Bevel,
                            });
                            match &stroke.content {
                                Texture::Solid(color) => {
                                    self.context.set_stroke_style_color(&color.to_rgba_color());
                                }
                                Texture::LinearGradient(gradient) => {
                                    let canvas_gradient = self.context.create_linear_gradient(
                                        gradient.start.x,
                                        gradient.start.y,
                                        gradient.end.x,
                                        gradient.end.y,
                                    );
                                    gradient.stops.iter().for_each(|stop| {
                                        canvas_gradient
                                            .add_color_stop(
                                                stop.offset,
                                                &stop.color.to_rgba_color(),
                                            )
                                            .unwrap();
                                    });
                                    self.context.set_stroke_style_gradient(&canvas_gradient);
                                }
                                Texture::Image(image) => {
                                    let pattern: CanvasPattern = js! {
                                        @{&self.context}.createPattern(@{image.deref()}, "no-repeat");
                                    }
                                    .try_into()
                                    .unwrap();
                                    self.context.set_stroke_style_pattern(&pattern);
                                    self.context.scale(self.pixel_ratio, self.pixel_ratio)
                                }
                                Texture::RadialGradient(gradient) => {
                                    let canvas_gradient = self
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
                                            .add_color_stop(
                                                stop.offset,
                                                &stop.color.to_rgba_color(),
                                            )
                                            .unwrap();
                                    });
                                    self.context.set_stroke_style_gradient(&canvas_gradient);
                                }
                            }
                            self.context.set_line_width(f64::from(stroke.width));
                            if let Texture::Image(_image) = &stroke.content {
                                self.context.scale(1. / self.pixel_ratio, 1. / self.pixel_ratio);
                            }
                            self.context.stroke();
                            if let Texture::Image(_image) = &stroke.content {
                                self.context.scale(self.pixel_ratio, self.pixel_ratio);
                            }
                        }
                        None => {}
                    }
                    match &entity.fill {
                        Some(fill) => {
                            match &fill.content {
                                Texture::Solid(color) => {
                                    self.context.set_fill_style_color(&color.to_rgba_color());
                                }
                                Texture::Image(image) => {
                                    let pattern: CanvasPattern = js! {
                                        return @{&self.context}.createPattern(@{image.deref()}, "no-repeat");
                                    }
                                    .try_into()
                                    .unwrap();
                                    self.context.scale(1. / self.pixel_ratio, 1. / self.pixel_ratio);
                                    self.context.set_fill_style_pattern(&pattern);
                                    self.context.scale(self.pixel_ratio, self.pixel_ratio);
                                }
                                Texture::LinearGradient(gradient) => {
                                    let canvas_gradient = self.context.create_linear_gradient(
                                        gradient.start.x,
                                        gradient.start.y,
                                        gradient.end.x,
                                        gradient.end.y,
                                    );
                                    gradient.stops.iter().for_each(|stop| {
                                        canvas_gradient
                                            .add_color_stop(
                                                stop.offset,
                                                &stop.color.to_rgba_color(),
                                            )
                                            .unwrap();
                                    });
                                    self.context.set_fill_style_gradient(&canvas_gradient);
                                }
                                Texture::RadialGradient(gradient) => {
                                    let canvas_gradient = self
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
                                            .add_color_stop(
                                                stop.offset,
                                                &stop.color.to_rgba_color(),
                                            )
                                            .unwrap();
                                    });
                                    self.context.set_fill_style_gradient(&canvas_gradient);
                                }
                            }
                            if let Texture::Image(_image) = &fill.content {
                                self.context.scale(1. / self.pixel_ratio, 1. / self.pixel_ratio);
                            }
                            self.context.fill(FillRule::NonZero);
                            if let Texture::Image(_image) = &fill.content {
                                self.context.scale(self.pixel_ratio, self.pixel_ratio);
                            }
                        }
                        None => {}
                    }
                });
            };
            let orientation: Transform;
            let content: Iter<Path<CanvasImage>>;
            match object {
                Object::Dynamic(object) => {
                    orientation = object.orientation();
                    let _content = object.render();
                    content = _content.iter();
                    draw(orientation, content);
                }
                Object::Static(object) => {
                    orientation = object.orientation.clone();
                    content = object.content.iter();
                    draw(orientation, content);
                }
            }
        });
    }
}

impl DynamicObject<CanvasImage> for CanvasFrame {
    fn orientation(&self) -> Transform {
        Transform::default()
    }
    fn render(&self) -> Cow<[Path<CanvasImage>]> {
        self.draw();
        let size = self.size.get();
        Cow::from(vec![Path {
            orientation: Transform::default(),
            fill: Some(Fill {
                content: Texture::Image(Box::new(self.canvas.clone())),
            }),
            shadow: None,
            stroke: None,
            closed: true,
            segments: vec![
                Segment::LineTo((0., 0.).into()),
                Segment::LineTo((0., size.y).into()),
                Segment::LineTo(size),
                Segment::LineTo((size.x, 0.).into()),
            ],
        }])
    }
}

impl Frame for CanvasFrame {
    type Image = CanvasImage;
    fn add<U>(&mut self, object: U) where U: Into<Object<CanvasImage>> {
        self.contents.push(object.into());
    }
    fn set_viewport(&self, viewport: Rect) {
        self.viewport.set(viewport);
    }
    fn resize<T>(&self, size: T) where T: Into<Vector> {
        let size = size.into();
        self.size.set(size);
        self.canvas
            .set_height((size.y * self.pixel_ratio) as u32);
        self.canvas
            .set_width((size.x * self.pixel_ratio) as u32);
    }
    fn get_size(&self) -> Vector {
        self.size.get()
    }
    fn to_image(&self) -> Box<CanvasImage> {
        self.draw();
        Box::new(self.canvas.clone())
    }
}

struct Canvas {
    state: Rc<RefCell<CanvasState>>,
}

struct CanvasState {
    root_frame: Option<CanvasFrame>,
    size: ObserverCell<Vector>,
}

impl Rasterizer for Canvas {
    type Image = CanvasImage;
    fn rasterize<'a, T>(&self, input: T) -> Self::Image where T: Into<Rasterizable<'a>> {
        let update_text_style = |context: &CanvasRenderingContext2d, input: &Text| {
            context.set_font((match input.font {
                Font::SystemFont => {
                    format!(r#"{} {} {}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol""#, if input.italic { "italic " } else { "" }, match input.weight {
                        Weight::Normal => "400",
                        Weight::Bold => "500",
                        Weight::Heavy => "700",
                        Weight::Thin => "200",
                        Weight::Light => "200",
                        Weight::Hairline => "100"
                    }, (f64::from(input.size) * window().device_pixel_ratio()) as u32)
                }
            }).as_str());
            context.set_text_align(match input.align {
                Align::Center => TextAlign::Center,
                Align::End => TextAlign::End,
                Align::Start => TextAlign::Start,
            });
            context.set_text_baseline(TextBaseline::Hanging);
            context.set_fill_style_color(&input.color.to_rgba_color());
        };
        let input = input.into();
        let canvas: CanvasElement = document().create_element("canvas").unwrap().try_into().unwrap();
        let dpr = window().device_pixel_ratio();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        match input {
            Rasterizable::Text(input) => {
                let mut lines: Vec<String> = input.content.split('\n').map(std::borrow::ToOwned::to_owned).collect();
                canvas.set_height((f64::from(input.line_height) * dpr) as u32 * ((lines.len() - 1).max(0) as u32) + (f64::from(input.size) * dpr) as u32);
                match input.max_width {
                    None => {
                        update_text_style(&context, &input);
                        canvas.set_width(context.measure_text(input.content).unwrap().get_width() as u32);
                    }
                    Some(max_width) => {
                        canvas.set_width((f64::from(max_width) * dpr) as u32);
                    }
                }
                update_text_style(&context, &input);
                if let Some(max_width) = input.max_width {
                    lines = match input.wrap {
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
                                            if context.measure_text(&(test_string.clone() + word)).unwrap().get_width() <= f64::from(max_width) * dpr {
                                                test_string += &format!(" {}", word);
                                            } else {
                                                test_string = test_string.trim().to_owned();
                                                wrapped_lines.push(test_string);
                                                lines.push(words.iter().cloned().skip(index).collect::<Vec<&str>>().join(" "));
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
                            canvas.set_height((f64::from(input.line_height) * dpr) as u32 * ((wrapped_lines.len() - 1).max(0) as u32) + (f64::from(input.size) * dpr) as u32);
                            update_text_style(&context, &input);
                            wrapped_lines
                        }
                        _ => lines
                    }
                }
                for (index, line) in lines.iter().enumerate() {
                    context.fill_text(line, 0., ((f64::from(input.line_height) * dpr) as u32 * index as u32).into(), None);
                }
            }
        }
        canvas
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

impl ContextGraphics for Canvas {}

impl ContextualGraphics for Canvas {
    type Context = Canvas;
    fn run(self, root: CanvasFrame) -> Self::Context {
        {
            let mut state = self.state.borrow_mut();
            root.show();
            state.root_frame = Some(root);
            let cloned = self.clone();
            window().request_animation_frame(move |delta| {
                cloned.animate(delta);
            });
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
    fn animate(&self, _delta: f64) {
        let state = self.state.borrow_mut();
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
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
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

pub fn new() -> impl ContextualGraphics {
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
        state: Rc::new(RefCell::new(CanvasState {
            size: ObserverCell::new((body.offset_width().into(), body.offset_height().into()).into()),
            root_frame: None,
        })),
    };

    let gfx_resize = gfx.clone();

    window().add_event_listener(move |_: ResizeEvent| {
        let state = gfx_resize.state.borrow();
        let body = document().body().unwrap();
        state.size.set((body.offset_width().into(), body.offset_height().into()).into());
    });

    gfx
}
