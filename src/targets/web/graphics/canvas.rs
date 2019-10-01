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

use std::cell::RefCell;
use std::rc::Rc;

use std::any::Any;

type CanvasImage = web_sys::HtmlCanvasElement;

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
        let canvas: web_sys::HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .expect("Could not create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        canvas.set_width(texture.format.width);
        canvas.set_height(texture.format.height);
        let image = context
            .create_image_data_with_sw_and_sh(
                f64::from(texture.format.width),
                f64::from(texture.format.height),
            )
            .unwrap();
        context.put_image_data(&image, 0., 0.).unwrap();
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
    state: Rc<RefCell<CanvasObjectState>>,
}

impl CanvasObject {
    fn new(content: Rasterizable, orientation: Transform2, depth: u32) -> CanvasObject {
        CanvasObject {
            state: Rc::new(RefCell::new(CanvasObjectState {
                orientation,
                content,
                depth,
            })),
        }
    }
}

impl Object for CanvasObject {
    fn get_transform(&self) -> Transform2 {
        self.state.borrow().orientation
    }
    fn apply_transform(&mut self, transform: Transform2) {
        self.state.borrow_mut().orientation.transform(transform);
    }
    fn set_transform(&mut self, transform: Transform2) {
        self.state.borrow_mut().orientation = transform;
    }
    fn set_depth(&mut self, depth: u32) {
        self.state.borrow_mut().depth = depth;
    }
    fn get_depth(&self) -> u32 {
        self.state.borrow().depth
    }
    fn update(&mut self, input: Rasterizable) {
        self.state.borrow_mut().content = input;
    }
    fn box_clone(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

struct CanvasFrameState {
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
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
    state: Rc<RefCell<CanvasFrameState>>,
}

impl CanvasFrame {
    fn new_raw(pixel_ratio: f64) -> CanvasFrame {
        let canvas: web_sys::HtmlCanvasElement = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .expect("Could not create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        CanvasFrame {
            state: Rc::new(RefCell::new(CanvasFrameState {
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
        let window = web_sys::window().unwrap();
        let canvas: web_sys::HtmlCanvasElement = window
            .document()
            .unwrap()
            .create_element("canvas")
            .expect("Could not create canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let clip_frame = Some(CanvasFrame::new_raw(window.device_pixel_ratio()));
        Box::new(CanvasFrame {
            state: Rc::new(RefCell::new(CanvasFrameState {
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
        let state = self.state.borrow();
        if web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(".root")
            .expect("Could not select for root class")
            .is_some()
        {
            panic!("A graphics context has already been started");
        }
        state.canvas.class_list().add_1("root").unwrap();
    }
    fn draw_shadows(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.borrow();
        for shadow in &entity.shadows {
            state.context.restore();
            state.context.save();
            state
                .context
                .transform(
                    matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
                )
                .unwrap();
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
            state
                .context
                .translate(scale_offset.x, scale_offset.y)
                .unwrap();
            state.context.scale(scale.x, scale.y).unwrap();
            state.context.move_to(-offset.x, -offset.y);
            state
                .context
                .translate(-offset.x / scale.x, -offset.y / scale.y)
                .unwrap();
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
            state
                .context
                .set_fill_style(&"rgba(255,255,255,1)".to_owned().into());
            state.context.fill();
        }
        state.context.restore();
        state.context.save();
        state
            .context
            .transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            )
            .unwrap();
        state.context.set_shadow_color("rgba(255,255,255,0)");
    }
    fn draw_path_clipped(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.borrow();
        if !entity.clip_segments.is_empty() && state.clip_frame.is_some() {
            state.context.restore();
            state.context.save();
            state
                .context
                .transform(
                    matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
                )
                .unwrap();
            state
                .context
                .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio)
                .unwrap();
            let frame = state.clip_frame.as_ref().unwrap();
            let mut matrix = matrix;
            matrix[4] *= state.pixel_ratio;
            matrix[5] *= state.pixel_ratio;
            matrix[3] = state.pixel_ratio;
            matrix[0] = state.pixel_ratio;
            frame.draw_path(matrix, entity);
            frame.composite_clip(matrix, entity);
            let el = frame.element();
            state.context.set_image_smoothing_enabled(false);
            state
                .context
                .draw_image_with_html_canvas_element(&el, -matrix[4], -matrix[5])
                .unwrap();
            frame.clear();
        } else {
            self.draw_path(matrix, entity);
        }
    }
    fn clear(&self) {
        let state = self.state.borrow();
        state.context.clear_rect(-1000., -1000., 2000., 2000.);
    }
    fn composite_clip(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.borrow();
        state.context.restore();
        state.context.save();
        state
            .context
            .transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            )
            .unwrap();
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
        state
            .context
            .set_global_composite_operation("destination-in")
            .unwrap();
        state.context.fill();
    }
    fn draw_path(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.borrow();
        state.context.restore();
        state.context.save();
        state
            .context
            .transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            )
            .unwrap();
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
                    StrokeCapType::Butt => "butt",
                    StrokeCapType::Round => "round",
                });
                state.context.set_line_join(match &stroke.join {
                    StrokeJoinType::Miter => "miter",
                    StrokeJoinType::Round => "round",
                    StrokeJoinType::Bevel => "bevel",
                });
                match &stroke.content {
                    Texture::Solid(color) => {
                        state
                            .context
                            .set_stroke_style(&color.to_rgba_color().to_string().into());
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
                                .add_color_stop(stop.offset as f32, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_stroke_style(&canvas_gradient);
                    }
                    Texture::Image(image) => {
                        let pattern: web_sys::CanvasPattern =
                            match image.as_any().downcast::<CanvasImage>() {
                                Ok(as_image) => state
                                    .context
                                    .create_pattern_with_html_canvas_element(
                                        as_image.as_ref(),
                                        "no-repeat",
                                    )
                                    .expect("Could not create canvas pattern")
                                    .unwrap(),
                                Err(_) => {
                                    let as_image =
                                        CanvasImage::from_texture(image.box_clone().as_texture());
                                    state
                                        .context
                                        .create_pattern_with_html_canvas_element(
                                            as_image.as_ref(),
                                            "no-repeat",
                                        )
                                        .expect("Could not create canvas pattern")
                                        .unwrap()
                                }
                            };
                        state
                            .context
                            .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio)
                            .unwrap();
                        state.context.set_stroke_style(&pattern);
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
                                .add_color_stop(stop.offset as f32, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_stroke_style(&canvas_gradient);
                    }
                }
                state.context.set_line_width(f64::from(stroke.width));
                state.context.stroke();
                if let Texture::Image(_image) = &stroke.content {
                    state
                        .context
                        .scale(state.pixel_ratio, state.pixel_ratio)
                        .unwrap();
                }
            }
            None => {}
        }
        match &entity.fill {
            Some(fill) => {
                match &fill.content {
                    Texture::Solid(color) => {
                        state
                            .context
                            .set_fill_style(&color.to_rgba_color().to_string().into());
                    }
                    Texture::Image(image) => {
                        let pattern: web_sys::CanvasPattern =
                            match image.as_any().downcast::<CanvasImage>() {
                                Ok(as_image) => state
                                    .context
                                    .create_pattern_with_html_canvas_element(
                                        as_image.as_ref(),
                                        "no-repeat",
                                    )
                                    .expect("Could not create canvas pattern")
                                    .unwrap(),
                                Err(_) => {
                                    let as_image =
                                        CanvasImage::from_texture(image.box_clone().as_texture());
                                    state
                                        .context
                                        .create_pattern_with_html_canvas_element(
                                            as_image.as_ref(),
                                            "no-repeat",
                                        )
                                        .expect("Could not create canvas pattern")
                                        .unwrap()
                                }
                            };
                        state
                            .context
                            .scale(1. / state.pixel_ratio, 1. / state.pixel_ratio)
                            .unwrap();
                        state.context.set_fill_style(&pattern);
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
                                .add_color_stop(stop.offset as f32, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_fill_style(&canvas_gradient);
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
                                .add_color_stop(stop.offset as f32, &stop.color.to_rgba_color())
                                .unwrap();
                        });
                        state.context.set_fill_style(&canvas_gradient);
                    }
                }
                state.context.fill();
                if let Texture::Image(_image) = &fill.content {
                    state
                        .context
                        .scale(state.pixel_ratio, state.pixel_ratio)
                        .unwrap();
                }
            }
            None => {}
        }
    }
    fn update_text_style(&self, input: &Text) {
        let state = self.state.borrow();
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
            Align::Center => "center",
            Align::End => "end",
            Align::Start => "start",
        });
        state.context.set_text_baseline(match input.origin {
            Origin::Top => "top",
            Origin::Baseline => "alphabetic",
            Origin::Middle => "middle",
        });
        state
            .context
            .set_fill_style(&input.color.to_rgba_color().to_string().into());
    }
    fn fill_text_with_spacing(&self, text: &'_ str, position: Vector2, spacing: f64) {
        if text == "" {
            return;
        }
        let state = self.state.borrow();
        let mut full_width = state.context.measure_text(&text).unwrap().width();
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
                .fill_text(&head.to_string(), position.x, position.y)
                .expect("Failed to fill text");

            let shorter_width = if text == "" {
                0.
            } else {
                state.context.measure_text(&text).unwrap().width()
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
        let state = self.state.borrow();
        let mut full_width = state.context.measure_text(&text).unwrap().width();
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
                state.context.measure_text(&text).unwrap().width()
            };
            let character_width = full_width - shorter_width;
            spaced_width += character_width + spacing;
            full_width = shorter_width;
            text != ""
        } {}
        spaced_width - spacing
    }
    fn draw_text(&self, matrix: [f64; 6], input: &Text) {
        let state = self.state.borrow();
        state.context.restore();
        state.context.save();
        state
            .context
            .transform(
                matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5],
            )
            .unwrap();
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
                state
                    .context
                    .fill_text(line, 0., input.line_height * f64::from(index as u32))
                    .expect("Could not fill text");
            }
        }
    }
    fn element(&self) -> web_sys::HtmlCanvasElement {
        let state = self.state.borrow();
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
        let document = web_sys::window().unwrap().document().unwrap();
        let el = document
            .create_element("span")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let style = el.style();
        style.set_property("position", "fixed").unwrap();
        style.set_property("left", "-5000px").unwrap();
        style.set_property("top", "-5000px").unwrap();
        el.set_text_content(Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        style.set_property("font", &font).unwrap();
        document.body().unwrap().append_child(&el).unwrap();
        el.offset_height() as f64
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
        let mut state = self.state.borrow_mut();
        state.pixel_ratio = ratio;
    }
    fn draw(&self) {
        let state = self.state.borrow();
        let viewport = state.viewport;
        let size = state.size;
        state
            .context
            .set_transform(
                (size.x / viewport.size.x) * state.pixel_ratio,
                0.,
                0.,
                (size.y / viewport.size.y) * state.pixel_ratio,
                -viewport.position.x * state.pixel_ratio,
                -viewport.position.y * state.pixel_ratio,
            )
            .unwrap();
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
                let a = a.state.borrow();
                let b = b.state.borrow();
                a.depth.partial_cmp(&b.depth).unwrap()
            })
            .for_each(|object| {
                let object = object.state.borrow();
                let matrix = object.orientation.to_matrix();
                match &object.content {
                    Rasterizable::Path(path) => self.draw_path_clipped(matrix, &path),
                    Rasterizable::Text(input) => self.draw_text(matrix, &input),
                };
            });
    }
    fn show(&self) {
        let state = self.state.borrow();
        state.canvas.set_oncontextmenu(Some(
            Closure::wrap(Box::new(|event: web_sys::Event| {
                event.prevent_default();
                event.stop_propagation();
            }) as Box<dyn FnMut(_)>)
            .as_ref()
            .unchecked_ref(),
        ));
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&state.canvas)
            .unwrap();
    }
    fn add(&mut self, content: Content) -> Box<dyn Object> {
        let object = CanvasObject::new(content.content, content.transform, content.depth);
        let mut state = self.state.borrow_mut();
        state.contents.push(object.clone());
        Box::new(object)
    }
    fn set_viewport(&self, viewport: Rect) {
        let mut state = self.state.borrow_mut();
        state.viewport = viewport;
        if let Some(frame) = &state.clip_frame {
            frame.set_viewport(viewport);
        }
    }
    fn resize(&self, size: Vector2) {
        let mut state = self.state.borrow_mut();
        state.size = size;
        state.canvas.set_height((size.y * state.pixel_ratio) as u32);
        state.canvas.set_width((size.x * state.pixel_ratio) as u32);
        if let Some(frame) = &state.clip_frame {
            frame.resize(size);
        }
    }
    fn get_size(&self) -> Vector2 {
        let state = self.state.borrow();
        state.size
    }
    fn to_image(&self) -> Box<dyn ImageRepresentation> {
        let state = self.state.borrow();
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
    state: Rc<RefCell<CanvasState>>,
}

struct CanvasState {
    root_frame: Option<Box<dyn Frame>>,
    size: ObserverCell<Vector2>,
    input: web::input::Input,
    cb: Option<Box<dyn FnMut(Box<dyn ActiveCanvas>) + 'static>>,
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
        self.state.borrow().input.box_clone()
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
    fn run_with(self: Box<Self>, cb: Box<dyn FnMut(Box<dyn ActiveCanvas>) + 'static>) {
        {
            let mut state = self.state.borrow_mut();
            state.cb = Some(cb);
            state.root_frame.as_ref().unwrap().show();
            let cloned = self.clone();
            web_sys::window()
                .unwrap()
                .request_animation_frame(
                    Closure::wrap(Box::new(move |start_time: f64| {
                        cloned.animate(start_time, start_time);
                    }) as Box<dyn FnMut(f64)>)
                    .as_ref()
                    .unchecked_ref(),
                )
                .expect("Cannot register animation frame request");
        }
    }
}

impl InteractiveCanvas for Canvas {
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveCanvas> {
        {
            let mut state = self.state.borrow_mut();
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
        let mut state = self.state.borrow_mut();
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
        state.cb.as_mut().map(|cb| (cb)(Box::new(self.clone())));
        web_sys::window()
            .unwrap()
            .request_animation_frame(
                Closure::wrap(Box::new(move |new_start_time: f64| {
                    cloned.animate(new_start_time, start_time);
                }) as Box<dyn FnMut(f64)>)
                .as_ref()
                .unchecked_ref(),
            )
            .expect("Cannot register animation frame request");
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
        state: Rc::new(RefCell::new(CanvasState {
            size: ObserverCell::new(
                (body.offset_width().into(), body.offset_height().into()).into(),
            ),
            root_frame: None,
            input: web::input::Input::new(),
            cb: None,
        })),
    };

    let gfx_resize = gfx.clone();

    window.set_onresize(Some(
        Closure::wrap(Box::new(move || {
            let state = gfx_resize.state.borrow();
            state
                .size
                .set((body.offset_width().into(), body.offset_height().into()).into());
        }) as Box<dyn Fn()>)
        .as_ref()
        .unchecked_ref(),
    ));

    Box::new(gfx)
}
