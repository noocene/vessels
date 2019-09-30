use crate::graphics::path::{Path, Segment, StrokeCapType, StrokeJoinType, Texture};
use crate::graphics::text::{Align, Font, Origin, Text, Weight, Wrap};
use crate::graphics::{
    canvas::{
        ActiveCanvas, Canvas as VesselsCanvas, CanvasContext, Content, Frame, InactiveCanvas,
        InteractiveCanvas, Object, Rasterizable, Rasterizer,
    },
    Image, ImageRepresentation, LDRColor, Rect, Texture2, Transform2, Vector2,
};
use crate::input::{windowing::Event as WindowingEvent, Event, Input, Provider};
use crate::targets::web;
use crate::util::ObserverCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use itertools::Itertools;

use stdweb::traits::{IChildNode, IElement, IEvent, IEventTarget, INode};
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    event::{ContextMenuEvent, ResizeEvent},
    window, CanvasPattern, CanvasRenderingContext2d, FillRule, LineCap, LineJoin, TextAlign,
    TextBaseline,
};

use stdweb::web::html_element::CanvasElement;

use std::sync::{Arc, RwLock};

use std::ops::Deref;

use std::any::Any;

type CanvasImage = CanvasElement;

impl ImageRepresentation for CanvasImage {
    fn get_size(&self) -> Vector2 {
        let dpr = web_sys::window()
            .expect("Cannot access window")
            .device_pixel_ratio();
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
    fn as_texture(&self) -> Image<LDRColor, Texture2> {
        Image {
            pixels: vec![],
            format: Texture2 {
                height: 0,
                width: 0,
            },
        }
    }
    fn from_texture(texture: Image<LDRColor, Texture2>) -> CanvasImage {
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
    orientation: Transform2,
    content: Rasterizable,
    depth: u32,
}

#[derive(Clone)]
struct CanvasObject {
    state: Arc<RwLock<CanvasObjectState>>,
}

impl CanvasObject {
    fn new(content: Rasterizable, orientation: Transform2, depth: u32) -> CanvasObject {
        CanvasObject {
            state: Arc::new(RwLock::new(CanvasObjectState {
                orientation,
                content,
                depth,
            })),
        }
    }
}

impl Object for CanvasObject {
    fn get_transform(&self) -> Transform2 {
        self.state.read().unwrap().orientation
    }
    fn apply_transform(&mut self, transform: Transform2) {
        self.state.write().unwrap().orientation.transform(transform);
    }
    fn set_transform(&mut self, transform: Transform2) {
        self.state.write().unwrap().orientation = transform;
    }
    fn set_depth(&mut self, depth: u32) {
        self.state.write().unwrap().depth = depth;
    }
    fn get_depth(&self) -> u32 {
        self.state.read().unwrap().depth
    }
    fn update(&mut self, input: Rasterizable) {
        self.state.write().unwrap().content = input;
    }
    fn box_clone(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

struct CanvasFrameState {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
    contents: Vec<CanvasObject>,
    pixel_ratio: f64,
    viewport: Rect,
    size: Vector2,
    clip_frame: Option<CanvasFrame>,
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
    fn new_raw(pixel_ratio: f64) -> CanvasFrame {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        CanvasFrame {
            state: Arc::new(RwLock::new(CanvasFrameState {
                canvas,
                pixel_ratio,
                context,
                contents: vec![],
                size: (1., 1.).into(),
                viewport: Rect::default(),
                clip_frame: None,
            })),
        }
    }
    fn new() -> Box<CanvasFrame> {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let clip_frame = Some(CanvasFrame::new_raw(
            web_sys::window()
                .expect("Cannot access window")
                .device_pixel_ratio(),
        ));
        Box::new(CanvasFrame {
            state: Arc::new(RwLock::new(CanvasFrameState {
                canvas,
                pixel_ratio: 0.,
                context,
                contents: vec![],
                size: Vector2::default(),
                viewport: Rect::default(),
                clip_frame,
            })),
        })
    }
    fn set_root(&self) {
        let state = self.state.read().unwrap();
        js! {
            let elem = document.querySelector(".root");
            if (elem !== null) {
                @{|| {panic!("A graphics context has already been started")}}();
            }
        };
        state.canvas.class_list().add("root").unwrap();
    }
    fn draw_shadows(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        for shadow in &entity.shadows {
            state.context.restore();
            state.context.save();
            state.context.transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            );
            let spread = shadow.spread * 2.;
            let size = entity.bounds().size;
            let scale = (size + spread) / size;
            state.context.begin_path();
            let segments = entity.segments.iter();
            let offset: Vector2 = (
                state.viewport.size.x + state.viewport.position.x,
                state.viewport.size.y + state.viewport.position.y,
            )
                .into();
            let new_size = size + spread;
            let scale_offset = (size - new_size) / 2.;
            state.context.translate(scale_offset.x, scale_offset.y);
            state.context.scale(scale.x, scale.y);
            state.context.move_to(-offset.x, -offset.y);
            state
                .context
                .translate(-offset.x / scale.x, -offset.y / scale.y);
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
            state
                .context
                .set_shadow_blur(shadow.blur * state.pixel_ratio);
            state
                .context
                .set_shadow_color(&shadow.color.to_rgba_color());
            state
                .context
                .set_shadow_offset_x((shadow.offset.x + offset.x) * state.pixel_ratio);
            state
                .context
                .set_shadow_offset_y((shadow.offset.y + offset.y) * state.pixel_ratio);
            state.context.set_fill_style_color("rgba(255,255,255,1)");
            state.context.fill(FillRule::NonZero);
        }
        state.context.restore();
        state.context.save();
        state.context.transform(
            matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
        );
        state.context.set_shadow_color("rgba(255,255,255,0)");
    }
    fn draw_path_clipped(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        if !entity.clip_segments.is_empty() && state.clip_frame.is_some() {
            state.context.restore();
            state.context.save();
            state.context.transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            );
            state
                .context
                .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio);
            let frame = state.clip_frame.as_ref().unwrap();
            let mut matrix = matrix;
            matrix[4] *= state.pixel_ratio;
            matrix[5] *= state.pixel_ratio;
            matrix[3] = state.pixel_ratio;
            matrix[0] = state.pixel_ratio;
            frame.draw_path(matrix, entity);
            frame.composite_clip(matrix, entity);
            let el = frame.element();
            js! {
                @{&state.context}.imageSmoothingEnabled = false;
                @{&state.context}.drawImage(@{&el}, @{-matrix[4]}, @{-matrix[5]});
            }
            frame.clear();
        } else {
            self.draw_path(matrix, entity);
        }
    }
    fn clear(&self) {
        let state = self.state.read().unwrap();
        state.context.clear_rect(-1000., -1000., 2000., 2000.);
    }
    fn composite_clip(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        state.context.restore();
        state.context.save();
        state.context.transform(
            matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
        );
        state.context.begin_path();
        entity
            .clip_segments
            .iter()
            .for_each(|segment| match segment {
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
        js! {
            @{&state.context}.globalCompositeOperation = "destination-in";
        };
        state.context.fill(FillRule::NonZero);
    }
    fn draw_path(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        state.context.restore();
        state.context.save();
        state.context.transform(
            matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
        );
        self.draw_shadows(matrix, &entity);
        state.context.begin_path();
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
                    format!(r#"{} {} {}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple LDRColor Emoji", "Segoe UI Emoji", "Segoe UI Symbol""#, if input.italic { "italic " } else { "" }, match input.weight {
                        Weight::Normal => "400",
                        Weight::Medium => "500",
                        Weight::SemiBold => "600",
                        Weight::Bold => "700",
                        Weight::ExtraBold => "800",
                        Weight::Heavy => "900",
                        Weight::Thin => "200",
                        Weight::Light => "300",
                        Weight::Hairline => "100"
                    }, input.size)
                }
            }).as_str());
        state.context.set_text_align(match input.align {
            Align::Center => TextAlign::Center,
            Align::End => TextAlign::End,
            Align::Start => TextAlign::Start,
        });
        state.context.set_text_baseline(match input.origin {
            Origin::Top => TextBaseline::Top,
            Origin::Baseline => TextBaseline::Alphabetic,
            Origin::Middle => TextBaseline::Middle,
        });
        state
            .context
            .set_fill_style_color(&input.color.to_rgba_color());
    }
    fn fill_text_with_spacing(&self, text: &'_ str, position: Vector2, spacing: f64) {
        if text == "" {
            return;
        }
        let state = self.state.read().unwrap();
        let mut full_width = state.context.measure_text(&text).unwrap().get_width();
        let mut position = position;
        let mut text = text.to_owned();
        let mut text_iter = text.chars();
        while {
            let head = text_iter.next().unwrap();
            text = text_iter
                .map(|character| character.to_string())
                .collect::<Vec<String>>()
                .join("");
            text_iter = text.chars();
            state
                .context
                .fill_text(&head.to_string(), position.x, position.y, None);

            let shorter_width = if text == "" {
                0.
            } else {
                state.context.measure_text(&text).unwrap().get_width()
            };
            let character_width = full_width - shorter_width;
            position.x += character_width + spacing;
            full_width = shorter_width;
            text != ""
        } {}
    }
    fn measure_text_with_spacing(&self, text: &'_ str, spacing: f64) -> f64 {
        if text == "" {
            return 0.;
        }
        let state = self.state.read().unwrap();
        let mut full_width = state.context.measure_text(&text).unwrap().get_width();
        if spacing == 0. {
            return full_width;
        }
        let mut spaced_width = 0.;
        let mut text = text.to_owned();
        let mut text_iter = text.chars();
        while {
            text_iter.next().unwrap();
            text = text_iter
                .map(|character| character.to_string())
                .collect::<Vec<String>>()
                .join("");
            text_iter = text.chars();
            let shorter_width = if text == "" {
                0.
            } else {
                state.context.measure_text(&text).unwrap().get_width()
            };
            let character_width = full_width - shorter_width;
            spaced_width += character_width + spacing;
            full_width = shorter_width;
            text != ""
        } {}
        spaced_width - spacing
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
            if input.letter_spacing != 0. {
                self.fill_text_with_spacing(
                    line,
                    (0., input.line_height * f64::from(index as u32)).into(),
                    input.letter_spacing,
                );
            } else {
                state.context.fill_text(
                    line,
                    0.,
                    input.line_height * f64::from(index as u32),
                    None,
                );
            }
        }
    }
    fn element(&self) -> CanvasElement {
        let state = self.state.read().unwrap();
        state.canvas.clone()
    }
    fn measure_text_height(&self, input: Text) -> f64 {
        let font = match input.font {
            Font::SystemFont => {
                format!(r#"{} {} {}px -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple LDRColor Emoji", "Segoe UI Emoji", "Segoe UI Symbol""#, if input.italic { "italic " } else { "" }, match input.weight {
                    Weight::Normal => "400",
                    Weight::Medium => "500",
                    Weight::SemiBold => "600",
                    Weight::Bold => "700",
                    Weight::ExtraBold => "800",
                    Weight::Heavy => "900",
                    Weight::Thin => "200",
                    Weight::Light => "300",
                    Weight::Hairline => "100"
                }, input.size)
            }
        };
        (js! {
            let el = document.createElement("span");
            el.style.position = "fixed";
            el.style.left = "-5000px";
            el.style.top = "-5000px";
            el.textContent = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
            el.style.font = @{font};
            document.body.appendChild(el);
            let offsetHeight = el.offsetHeight;
            el.remove();
            return offsetHeight;
        })
        .try_into()
        .unwrap()
    }
    fn wrap_text(&self, input: &Text) -> Vec<String> {
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
                                if self.measure_text_with_spacing(
                                    &(test_string.clone() + word),
                                    input.letter_spacing,
                                ) <= input.max_width.unwrap()
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

impl Frame for CanvasFrame {
    fn set_pixel_ratio(&self, ratio: f64) {
        let mut state = self.state.write().unwrap();
        state.pixel_ratio = ratio;
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
        state
            .contents
            .iter()
            .sorted_by(|a, b| {
                let a = a.state.read().unwrap();
                let b = b.state.read().unwrap();
                a.depth.partial_cmp(&b.depth).unwrap()
            })
            .for_each(|object| {
                let object = object.state.read().unwrap();
                let matrix = object.orientation.to_matrix();
                match &object.content {
                    Rasterizable::Path(path) => self.draw_path_clipped(matrix, &path),
                    Rasterizable::Text(input) => self.draw_text(matrix, &input),
                };
            });
    }
    fn show(&self) {
        let state = self.state.read().unwrap();
        state.canvas.add_event_listener(|event: ContextMenuEvent| {
            event.prevent_default();
            event.stop_propagation();
        });
        document().body().unwrap().append_child(&state.canvas);
    }
    fn add(&mut self, content: Content) -> Box<dyn Object> {
        let object = CanvasObject::new(content.content, content.transform, content.depth);
        let mut state = self.state.write().unwrap();
        state.contents.push(object.clone());
        Box::new(object)
    }
    fn set_viewport(&self, viewport: Rect) {
        let mut state = self.state.write().unwrap();
        state.viewport = viewport;
        if let Some(frame) = &state.clip_frame {
            frame.set_viewport(viewport);
        }
    }
    fn resize(&self, size: Vector2) {
        let mut state = self.state.write().unwrap();
        state.size = size;
        state.canvas.set_height((size.y * state.pixel_ratio) as u32);
        state.canvas.set_width((size.x * state.pixel_ratio) as u32);
        if let Some(frame) = &state.clip_frame {
            frame.resize(size);
        }
    }
    fn get_size(&self) -> Vector2 {
        let state = self.state.read().unwrap();
        state.size
    }
    fn to_image(&self) -> Box<dyn ImageRepresentation> {
        let state = self.state.read().unwrap();
        self.draw();
        Box::new(state.canvas.clone())
    }
    fn measure(&self, input: Rasterizable) -> Vector2 {
        match input {
            Rasterizable::Text(input) => {
                self.update_text_style(&input);
                let origin = input.origin;
                let mut size: Vector2 = if input.max_width.is_some() {
                    let lines = self.wrap_text(&input);
                    (
                        input.max_width.unwrap(),
                        (f64::from((lines.len() - 1).max(0) as u32) * input.line_height)
                            + input.size,
                    )
                        .into()
                } else {
                    (
                        self.measure_text_with_spacing(&input.content, input.letter_spacing),
                        self.measure_text_height(*input),
                    )
                        .into()
                };
                if origin == Origin::Middle {
                    size.y = 0.;
                }
                size
            }
            Rasterizable::Path(input) => input.bounds().size,
        }
    }
    fn box_clone(&self) -> Box<dyn Frame> {
        Box::new(CanvasFrame {
            state: self.state.clone(),
        })
    }
    fn as_any(&self) -> Box<dyn Any> {
        Box::new(CanvasFrame {
            state: self.state.clone(),
        })
    }
}

#[derive(Clone)]
struct Canvas {
    state: Arc<RwLock<CanvasState>>,
}

struct CanvasState {
    root_frame: Option<Box<dyn Frame>>,
    size: ObserverCell<Vector2>,
    input: web::input::Input,
}

impl Rasterizer for Canvas {
    fn rasterize(&self, input: Rasterizable, size: Vector2) -> Box<dyn ImageRepresentation> {
        let mut frame = CanvasFrame::new();
        if let Rasterizable::Text(text) = &input {
            match &text.origin {
                Origin::Top => {
                    frame.set_viewport(Rect::new(Vector2::default(), size));
                }
                Origin::Baseline => {
                    frame.set_viewport(Rect::new((0., -size.y), size));
                }
                Origin::Middle => {
                    frame.set_viewport(Rect::new((0., -size.y / 2.), size));
                }
            }
        }
        frame.resize(size);
        frame.add(input.into());
        frame.draw();
        Box::new(frame.element())
    }
}

impl Provider for Canvas {
    fn input(&self) -> Box<dyn Input> {
        self.state.read().unwrap().input.box_clone()
    }
}

impl CanvasContext for Canvas {}

impl ActiveCanvas for Canvas {
    fn box_clone(&self) -> Box<dyn ActiveCanvas> {
        Box::new(self.clone())
    }
}

impl InactiveCanvas for Canvas {
    fn run(self: Box<Self>) {
        self.run_with(Box::new(|_| {}));
    }
    fn run_with(self: Box<Self>, mut cb: Box<dyn FnMut(Box<dyn ActiveCanvas>) + Send + 'static>) {
        {
            let state = self.state.read().unwrap();
            state.root_frame.as_ref().unwrap().show();
            let cloned = self.clone();
            window().request_animation_frame(move |start_time| {
                cloned.animate(start_time, start_time);
            });
        }
        (cb)(self);
    }
}

impl InteractiveCanvas for Canvas {
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveCanvas> {
        {
            let mut state = self.state.write().unwrap();
            let size = state.size.get();
            let frame = root.as_any().downcast::<CanvasFrame>().unwrap();
            frame.set_root();
            root.resize(size);
            root.set_viewport(Rect::new((0., 0.), size));
            state.root_frame = Some(root);
        }
        self
    }
}

impl VesselsCanvas for Canvas {
    fn frame(&self) -> Box<dyn Frame> {
        let frame = CanvasFrame::new();
        frame.set_pixel_ratio(
            web_sys::window()
                .expect("Cannot access window")
                .device_pixel_ratio(),
        );
        frame
    }
}

impl Canvas {
    fn animate(&self, start_time: f64, last_start_time: f64) {
        let state = self.state.read().unwrap();
        state.input.send(Event::Windowing(WindowingEvent::Redraw(
            start_time - last_start_time,
        )));
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

pub(crate) fn new() -> Box<dyn InteractiveCanvas> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    if document
        .query_selector(".root")
        .expect("Could not select for root class")
        .is_some()
    {
        panic!("A graphics context has already been started");
    }
    let head = document.head().unwrap();
    head.append_child(
        &document
            .create_element("title")
            .expect("Could not create title"),
    )
    .expect("Could not append title");
    let style = document
        .create_element("style")
        .expect("Could not create style");
    style.set_inner_html(
        r#"
    body, html, canvas.root {
        height: 100%;
    }
    body {
        margin: 0;
        overflow: hidden;
    }
    canvas {
        display: none;
    }
    canvas.root {
        width: 100%;
        display: initial;
    }
                "#,
    );
    head.append_child(&style).expect("Could not append style");

    let body = document.body().unwrap();

    let gfx = Canvas {
        state: Arc::new(RwLock::new(CanvasState {
            size: ObserverCell::new(
                (body.offset_width().into(), body.offset_height().into()).into(),
            ),
            root_frame: None,
            input: web::input::Input::new(),
        })),
    };

    let gfx_resize = gfx.clone();

    window
        .add_event_listener_with_callback(
            "resize",
            Closure::wrap(Box::new(move || {
                let state = gfx_resize.state.read().unwrap();
                state
                    .size
                    .set((body.offset_width().into(), body.offset_height().into()).into());
            }) as Box<dyn Fn()>)
            .as_ref()
            .unchecked_ref(),
        )
        .expect("Cannot register resize event listener");

    Box::new(gfx)
}
