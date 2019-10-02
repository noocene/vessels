use super::cm::Profile;
use crate::graphics::path::{Path, Segment, StrokeCapType, StrokeJoinType, Texture};
use crate::graphics::text::{Origin, Text, Weight, Wrap};
use crate::graphics::{
    canvas::{
        ActiveCanvas, Canvas, CanvasContext, Content, Frame, InactiveCanvas, InteractiveCanvas,
        Object, Rasterizable, Rasterizer,
    },
    Image, ImageRepresentation, LDRColor, Rect, Texture2, Transform2, Vector2,
};
use crate::input::{
    keyboard::{self, Event as KeyboardEvent},
    mouse::{self, Event as MouseEvent},
    windowing::Event as WindowingEvent,
    Event, Input, Provider,
};
use crate::targets::native;
use crate::util::ObserverCell;
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::{task::AtomicTask, Async, Poll, Stream};

use std::{
    any::Any,
    cell::RefCell,
    ffi::{c_void, CString},
    ops::Deref,
    rc::Rc,
    time::SystemTime,
};

use glutin::{dpi::LogicalSize, ContextTrait};

use cairo::{
    Antialias, FontOptions, Format, Gradient, HintStyle, ImageSurface, LineCap, LineJoin,
    LinearGradient, Matrix, Operator, Pattern, RadialGradient, Status, SubpixelOrder,
};

use pango::{FontDescription, Layout, LayoutExt};

use gl::types::*;

use cairo_sys;

#[cfg(target_os = "windows")]
static SYSTEM_FONT: &str = "Segoe UI";
#[cfg(target_os = "macos")]
static SYSTEM_FONT: &str = "San Francisco";
#[cfg(target_os = "linux")]
static SYSTEM_FONT: &str = "DejaVu Sans";

struct CairoSurface(ImageSurface);

struct CairoContext(cairo::Context);

impl Deref for CairoSurface {
    type Target = ImageSurface;

    fn deref(&self) -> &ImageSurface {
        &self.0
    }
}

impl Deref for CairoContext {
    type Target = cairo::Context;

    fn deref(&self) -> &cairo::Context {
        &self.0
    }
}

struct CairoImage(Rc<RefCell<CairoSurface>>);

fn boxes_for_gauss(sigma: f64, n: u32) -> Vec<u32> {
    let nf = f64::from(n);
    let mut wl = ((12. * sigma * sigma / nf) + 1.).sqrt().floor() as u32;
    if wl % 2 == 0 {
        wl -= 1;
    }
    let wu = wl + 2;
    let wl = f64::from(wl);
    let m = ((12. * sigma * sigma - nf * wl * wl - 4. * nf * wl - 3. * nf) / (-4. * wl - 4.))
        .round() as u32;
    let mut sizes = vec![];
    for i in 0..n {
        sizes.push(if i < m { wl as u32 } else { wu })
    }
    sizes
}

impl CairoImage {
    fn new(surface: CairoSurface) -> CairoImage {
        CairoImage(Rc::new(RefCell::new(surface)))
    }
    fn box_blur(&self, data: &mut [[u8; 4]], width: u32, height: u32, radius: u32, channel: usize) {
        let mut target = vec![[0, 0, 0, 0]; data.len()];
        target.copy_from_slice(data);
        self.box_blur_h(
            data,
            &mut target,
            width as i32,
            height as i32,
            radius as i32,
            channel,
        );
        self.box_blur_t(
            &mut target,
            data,
            width as i32,
            height as i32,
            radius as i32,
            channel,
        );
    }
    fn box_blur_h(
        &self,
        source: &mut [[u8; 4]],
        target: &mut [[u8; 4]],
        width: i32,
        height: i32,
        radius: i32,
        channel: usize,
    ) {
        let iarr = 1. / f64::from(radius + radius + 1);
        for i in 0..height {
            let mut ti = i * width;
            let mut li = ti;
            let mut ri = ti + radius;
            let fv = i32::from(source[ti as usize][channel]);
            let lv = i32::from(source[(ti + width - 1) as usize][channel]);
            let mut val = (radius + 1) * fv;
            for j in 0..radius {
                val += i32::from(source[(ti + j) as usize][channel]);
            }
            for _ in 0..=radius {
                val += i32::from(source[ri as usize][channel]) - fv;
                ri += 1;
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                ti += 1;
            }
            for _ in radius + 1..width - radius {
                val += i32::from(source[ri as usize][channel])
                    - i32::from(source[li as usize][channel]);
                ri += 1;
                li += 1;
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                ti += 1;
            }
            for _ in width - radius..width {
                val += lv - i32::from(source[li as usize][channel]);
                li += 1;
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                ti += 1;
            }
        }
    }
    fn box_blur_t(
        &self,
        source: &mut [[u8; 4]],
        target: &mut [[u8; 4]],
        width: i32,
        height: i32,
        radius: i32,
        channel: usize,
    ) {
        let iarr = 1. / f64::from(radius + radius + 1);
        for i in 0..width {
            let mut ti = i;
            let mut li = ti;
            let mut ri = ti + radius * width;
            let fv = i32::from(source[ti as usize][channel]);
            let lv = i32::from(source[(ti + width * (height - 1)) as usize][channel]);
            let mut val = (radius + 1) * fv;
            for j in 0..radius {
                val += i32::from(source[(ti + j * width) as usize][channel]);
            }
            for _ in 0..=radius {
                val += i32::from(source[ri as usize][channel]) - fv;
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                ri += width;
                ti += width;
            }
            for _ in radius + 1..height - radius {
                val += i32::from(source[ri as usize][channel])
                    - i32::from(source[li as usize][channel]);
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                li += width;
                ti += width;
                ri += width;
            }
            for _ in height - radius..height {
                val += lv - i32::from(source[li as usize][channel]);
                target[ti as usize][channel] = (f64::from(val) * iarr).round() as u8;
                li += width;
                ti += width;
            }
        }
    }
    fn blur(&self, radius: f64) {
        let (width, height) = {
            let surface = &self.0.borrow_mut().0;
            (surface.get_width() as u32, surface.get_height() as u32)
        };
        let data: &mut [[u8; 4]] = unsafe {
            std::slice::from_raw_parts_mut(
                self.get_data_ptr() as *mut [u8; 4],
                (width * height) as usize,
            )
        };
        let boxes = boxes_for_gauss(radius, 3);
        for b in 0..=2 {
            self.box_blur(data, width, height, (boxes[b] - 1) / 2, 3);
        }
        unsafe { cairo_sys::cairo_surface_mark_dirty(self.0.borrow_mut().0.to_raw_none()) };
    }
    fn get_data_ptr(&self) -> *const c_void {
        let surface = &self.0.borrow_mut().0;
        unsafe {
            cairo_sys::cairo_surface_flush(surface.to_raw_none());
            match Status::from(cairo_sys::cairo_surface_status(surface.to_raw_none())) {
                Status::Success => (),
                _ => panic!("Cairo Surface borrow error!"),
            }
            if cairo_sys::cairo_image_surface_get_data(surface.to_raw_none()).is_null() {
                panic!("Cairo Surface borrow error!");
            }
            cairo_sys::cairo_image_surface_get_data(surface.to_raw_none()) as *const c_void
        }
    }
}

impl Clone for CairoImage {
    fn clone(&self) -> Self {
        CairoImage(self.0.clone())
    }
}

fn pixels_to_pango_points(pixels: f64) -> i32 {
    (pixels * 0.75 * f64::from(pango::SCALE)) as i32
}

fn pixels_to_pango_pixels(pixels: f64) -> i32 {
    (pixels * f64::from(pango::SCALE)) as i32
}

impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector2 {
        (
            f64::from(self.0.borrow_mut().get_width()),
            f64::from(self.0.borrow_mut().get_height()),
        )
            .into()
    }

    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(CairoImage(self.0.clone()))
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

    fn from_texture(texture: Image<LDRColor, Texture2>) -> CairoImage {
        CairoImage::new(CairoSurface(
            ImageSurface::create(
                Format::ARgb32,
                texture.format.width as i32,
                texture.format.height as i32,
            )
            .unwrap(),
        ))
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(CairoImage(self.0.clone()))
    }
}

struct CairoFrameState {
    context: RefCell<CairoContext>,
    contents: Vec<CairoObject>,
    viewport: Rect,
    color_profile: Option<Profile>,
    size: Vector2,
    pixel_ratio: f64,
}

struct CairoFrame {
    state: Rc<RefCell<CairoFrameState>>,
}

fn composite_clip(context: &CairoContext, entity: &Path) {
    if !entity.clip_segments.is_empty() {
        context.move_to(0., 0.);
        entity
            .clip_segments
            .iter()
            .for_each(|segment| match segment {
                Segment::LineTo(point) => {
                    context.line_to(point.x, point.y);
                }
                Segment::MoveTo(point) => {
                    context.move_to(point.x, point.y);
                }
                Segment::CubicTo(point, handle_1, handle_2) => {
                    context.curve_to(
                        handle_1.x, handle_1.y, handle_2.x, handle_2.y, point.x, point.y,
                    );
                }
                Segment::QuadraticTo(point, handle) => {
                    context.curve_to(handle.x, handle.y, handle.x, handle.y, point.x, point.y);
                }
            });
        context.set_source_rgb(0., 0., 0.);
        context.set_operator(Operator::DestIn);
        context.fill();
        context.set_operator(Operator::Over);
    }
}

fn draw_path(context: &CairoContext, entity: &Path, pixel_ratio: f64) {
    context.move_to(0., 0.);
    entity.segments.iter().for_each(|segment| match segment {
        Segment::LineTo(point) => {
            context.line_to(point.x, point.y);
        }
        Segment::MoveTo(point) => {
            context.move_to(point.x, point.y);
        }
        Segment::CubicTo(point, handle_1, handle_2) => {
            context.curve_to(
                handle_1.x, handle_1.y, handle_2.x, handle_2.y, point.x, point.y,
            );
        }
        Segment::QuadraticTo(point, handle) => {
            context.curve_to(handle.x, handle.y, handle.x, handle.y, point.x, point.y);
        }
    });
    if entity.closed {
        context.close_path();
    }
    match &entity.stroke {
        Some(stroke) => {
            context.set_line_cap(match &stroke.cap {
                StrokeCapType::Butt => LineCap::Butt,
                StrokeCapType::Round => LineCap::Round,
            });
            context.set_line_join(match &stroke.join {
                StrokeJoinType::Miter => LineJoin::Miter,
                StrokeJoinType::Round => LineJoin::Round,
                StrokeJoinType::Bevel => LineJoin::Bevel,
            });
            match &stroke.content {
                Texture::Solid(color) => {
                    context.set_source_rgba(
                        f64::from(color.r) / 255.,
                        f64::from(color.g) / 255.,
                        f64::from(color.b) / 255.,
                        f64::from(color.a) / 255.,
                    );
                }
                Texture::LinearGradient(gradient) => {
                    let canvas_gradient = LinearGradient::new(
                        gradient.start.x,
                        gradient.start.y,
                        gradient.end.x,
                        gradient.end.y,
                    );
                    gradient.stops.iter().for_each(|stop| {
                        canvas_gradient.add_color_stop_rgba(
                            stop.offset,
                            f64::from(stop.color.r) / 255.,
                            f64::from(stop.color.g) / 255.,
                            f64::from(stop.color.b) / 255.,
                            f64::from(stop.color.a) / 255.,
                        )
                    });
                    context.set_source(&Pattern::LinearGradient(canvas_gradient));
                }
                Texture::Image(image) => {
                    let pattern = image.as_any().downcast::<CairoImage>().unwrap();
                    let surface = &pattern.0.borrow_mut().0;
                    //TODO: coordinates here probd shouldn't be 0, 0
                    context.set_source_surface(surface, 0.0, 0.0);
                }
                Texture::RadialGradient(gradient) => {
                    let canvas_gradient = RadialGradient::new(
                        gradient.start.x,
                        gradient.start.y,
                        gradient.start_radius,
                        gradient.end.x,
                        gradient.end.y,
                        gradient.end_radius,
                    );
                    gradient.stops.iter().for_each(|stop| {
                        canvas_gradient.add_color_stop_rgba(
                            stop.offset,
                            f64::from(stop.color.r) / 255.,
                            f64::from(stop.color.g) / 255.,
                            f64::from(stop.color.b) / 255.,
                            f64::from(stop.color.a) / 255.,
                        );
                    });
                    context.set_source(&Pattern::RadialGradient(canvas_gradient));
                }
            }
            context.set_line_width(f64::from(stroke.width));
            if entity.fill.is_some() {
                context.stroke_preserve();
            } else {
                context.stroke();
            }
            if let Texture::Image(_image) = &stroke.content {
                context.scale(pixel_ratio, pixel_ratio);
            }
        }
        None => {}
    }
    match &entity.fill {
        Some(fill) => {
            match &fill.content {
                Texture::Solid(color) => {
                    context.set_source_rgba(
                        f64::from(color.r) / 255.,
                        f64::from(color.g) / 255.,
                        f64::from(color.b) / 255.,
                        f64::from(color.a) / 255.,
                    );
                }
                Texture::Image(image) => {
                    let pattern = image.as_any().downcast::<CairoImage>().unwrap();
                    let surface = &pattern.0.borrow_mut().0;
                    //TODO: coordinates here probd shouldn't be 0, 0
                    context.set_source_surface(surface, 0.0, 0.0);
                }
                Texture::LinearGradient(gradient) => {
                    let canvas_gradient = LinearGradient::new(
                        gradient.start.x,
                        gradient.start.y,
                        gradient.end.x,
                        gradient.end.y,
                    );
                    gradient.stops.iter().for_each(|stop| {
                        canvas_gradient.add_color_stop_rgba(
                            stop.offset,
                            f64::from(stop.color.r) / 255.,
                            f64::from(stop.color.g) / 255.,
                            f64::from(stop.color.b) / 255.,
                            f64::from(stop.color.a) / 255.,
                        )
                    });
                    context.set_source(&Pattern::LinearGradient(canvas_gradient));
                }
                Texture::RadialGradient(gradient) => {
                    let canvas_gradient = RadialGradient::new(
                        gradient.start.x,
                        gradient.start.y,
                        gradient.start_radius,
                        gradient.end.x,
                        gradient.end.y,
                        gradient.end_radius,
                    );
                    gradient.stops.iter().for_each(|stop| {
                        canvas_gradient.add_color_stop_rgba(
                            stop.offset,
                            f64::from(stop.color.r) / 255.,
                            f64::from(stop.color.g) / 255.,
                            f64::from(stop.color.b) / 255.,
                            f64::from(stop.color.a) / 255.,
                        );
                    });
                    context.set_source(&Pattern::RadialGradient(canvas_gradient));
                }
            }
            context.fill();
            if let Texture::Image(_image) = &fill.content {
                context.scale(pixel_ratio, pixel_ratio);
            }
        }
        None => {}
    }
}

impl CairoFrame {
    fn new() -> Box<CairoFrame> {
        let size = Vector2::default();
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        Box::new(CairoFrame {
            state: Rc::new(RefCell::new(CairoFrameState {
                context: RefCell::new(CairoContext(cairo::Context::new(&surface))),
                contents: vec![],
                size,
                color_profile: None,
                viewport: Rect {
                    size: Vector2::default(),
                    position: (0., 0.).into(),
                },
                pixel_ratio: 1.,
            })),
        })
    }
    fn surface(&self) -> Box<CairoImage> {
        self.draw();
        Box::new(CairoImage::new(CairoSurface(
            ImageSurface::from(self.state.borrow().context.borrow().get_target()).unwrap(),
        )))
    }
    fn set_color_profile(&self, profile: Profile) {
        let mut state = self.state.borrow_mut();
        state.color_profile = Some(profile);
    }
    fn layout_text(&self, entity: &Text) -> Layout {
        let state = self.state.borrow();
        let context = state.context.borrow_mut();
        let layout = pangocairo::functions::create_layout(&context).unwrap();
        layout.set_text(&entity.content);
        let mut font_options = FontOptions::new();
        font_options.set_antialias(Antialias::Gray);
        font_options.set_hint_style(HintStyle::Full);
        font_options.set_subpixel_order(SubpixelOrder::Rgb);
        context.set_font_options(&font_options);
        context.set_antialias(Antialias::Best);
        let mut font = FontDescription::new();
        font.set_absolute_size(f64::from(pixels_to_pango_pixels(entity.size)));
        font.set_family(SYSTEM_FONT);
        font.set_weight(match entity.weight {
            Weight::Bold => pango::Weight::Bold,
            Weight::Hairline => pango::Weight::Ultralight,
            Weight::Normal => pango::Weight::Normal,
            Weight::Heavy => pango::Weight::Heavy,
            Weight::Thin => pango::Weight::Semilight,
            Weight::Light => pango::Weight::Light,
            Weight::Medium => pango::Weight::Medium,
            Weight::ExtraBold => pango::Weight::Ultrabold,
            Weight::SemiBold => pango::Weight::Semibold,
        });
        layout.set_font_description(&font);
        if entity.max_width.is_some() {
            layout.set_width(pixels_to_pango_pixels(entity.max_width.unwrap()));
        }
        if let Wrap::Normal = entity.wrap {
            layout.set_wrap(pango::WrapMode::Word);
        }
        layout.set_spacing(pixels_to_pango_pixels(entity.line_height - entity.size));
        let attribute_list = pango::AttrList::new();
        attribute_list.insert(
            pango::Attribute::new_letter_spacing(pixels_to_pango_points(entity.letter_spacing))
                .unwrap(),
        );
        layout.set_attributes(&attribute_list);
        context.set_source_rgba(
            f64::from(entity.color.r) / 255.,
            f64::from(entity.color.g) / 255.,
            f64::from(entity.color.b) / 255.,
            f64::from(entity.color.a) / 255.,
        );
        pangocairo::functions::update_layout(&context, &layout);
        layout
    }
    fn measure_text(&self, entity: &Text) -> Vector2 {
        let layout = self.layout_text(entity);
        let size = layout.get_pixel_size();
        (f64::from(size.0), f64::from(size.1)).into()
    }
    fn draw_text(&self, matrix: [f64; 6], entity: &Text) {
        {
            let state = self.state.borrow();
            let context = state.context.borrow_mut();
            context.restore();
            context.save();
            context.transform(Matrix {
                xx: matrix[0],
                yx: matrix[2],
                xy: matrix[1],
                yy: matrix[3],
                x0: matrix[4],
                y0: matrix[5],
            });
        }
        let layout = self.layout_text(&entity);
        let state = self.state.borrow();
        let context = state.context.borrow_mut();
        match entity.origin {
            Origin::Baseline => {
                let baseline = layout.get_baseline();
                context.translate(0., f64::from(-baseline / pango::SCALE));
            }
            Origin::Middle => {
                let size = layout.get_pixel_size();
                context.translate(0., -(f64::from(size.1) / 2.));
            }
            _ => {}
        }
        pangocairo::functions::show_layout(&context, &layout);
    }

    fn draw_path(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.borrow();
        {
            let context = state.context.borrow_mut();
            context.restore();
            context.save();
            context.transform(Matrix {
                xx: matrix[0],
                yx: matrix[2],
                xy: matrix[1],
                yy: matrix[3],
                x0: matrix[4],
                y0: matrix[5],
            });
        }
        let context = state.context.borrow_mut();
        if entity.shadows.is_empty() && entity.clip_segments.is_empty() {
            draw_path(&context, entity, state.pixel_ratio);
        }
    }
}

impl Clone for CairoFrame {
    fn clone(&self) -> Self {
        CairoFrame {
            state: self.state.clone(),
        }
    }
}

impl Frame for CairoFrame {
    fn set_pixel_ratio(&self, ratio: f64) {
        let mut state = self.state.borrow_mut();
        state.pixel_ratio = ratio;
    }

    fn as_any(&self) -> Box<dyn Any> {
        Box::new(self.clone())
    }

    fn add(&mut self, content: Content) -> Box<dyn Object> {
        let object = CairoObject::new(
            content.content,
            content.transform,
            content.depth,
            self.state.borrow().color_profile.clone(),
        );
        let mut state = self.state.borrow_mut();
        state.contents.push(object.clone());
        Box::new(object)
    }

    fn set_viewport(&self, viewport: Rect) {
        let mut state = self.state.borrow_mut();
        state.viewport = viewport;
    }

    fn resize(&self, size: Vector2) {
        let mut state = self.state.borrow_mut();
        state.size = size;
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        state.context = RefCell::new(CairoContext(cairo::Context::new(&surface)));
    }

    fn get_size(&self) -> Vector2 {
        let state = self.state.borrow();
        state.size / state.pixel_ratio
    }

    fn to_image(&self) -> Box<dyn ImageRepresentation> {
        self.surface()
    }

    fn measure(&self, input: Rasterizable) -> Vector2 {
        match input {
            Rasterizable::Text(input) => {
                let mut size = self.measure_text(input.deref());
                if input.origin == Origin::Middle {
                    size.y = 0.;
                }
                size
            }
            Rasterizable::Path(input) => input.bounds().size,
        }
    }

    fn box_clone(&self) -> Box<dyn Frame> {
        Box::new(CairoFrame {
            state: self.state.clone(),
        })
    }

    fn show(&self) {}

    fn draw(&self) {
        let state = self.state.borrow();
        {
            let context = state.context.borrow_mut();
            context.set_source_rgb(1., 1., 1.);
            let viewport = state.viewport;
            let size = state.size;
            context.set_matrix(Matrix {
                xx: (size.x / viewport.size.x) * state.pixel_ratio,
                yx: 0.,
                xy: 0.,
                yy: -(size.y / viewport.size.y) * state.pixel_ratio,
                x0: -viewport.position.x * state.pixel_ratio,
                y0: -viewport.position.y * state.pixel_ratio + viewport.size.y,
            });
            context.rectangle(
                viewport.position.x,
                viewport.position.y,
                viewport.size.x,
                viewport.size.y,
            );
            context.fill();
            context.save();
        }
        state.contents.iter().for_each(|object| {
            let object_state = object.state.borrow();
            let matrix = object_state.orientation.to_matrix();
            object.redraw(state.pixel_ratio);
            (*object.cache_surface.borrow_mut())
                .iter()
                .for_each(|surface| {
                    let context = state.context.borrow_mut();
                    context.restore();
                    context.save();
                    context.transform(Matrix {
                        xx: matrix[0],
                        yx: matrix[2],
                        xy: matrix[1],
                        yy: matrix[3],
                        x0: matrix[4],
                        y0: matrix[5],
                    });
                    context.scale(1. / state.pixel_ratio, 1. / state.pixel_ratio);
                    context.set_source_surface(&surface.0.get_target(), surface.1.x, surface.1.y);
                    context.paint();
                });
            match &object_state.content {
                Rasterizable::Path(path) => self.draw_path(matrix, &path),
                Rasterizable::Text(input) => self.draw_text(matrix, &input),
            };
        });
    }
}

struct CairoObjectState {
    orientation: Transform2,
    content: Rasterizable,
    depth: u32,
    redraw: RefCell<bool>,
}

#[derive(Clone)]
struct CairoObject {
    state: Rc<RefCell<CairoObjectState>>,
    color_profile: Option<Profile>,
    cache_surface: Rc<RefCell<Option<(CairoContext, Vector2)>>>,
}

impl CairoObject {
    fn new(
        content: Rasterizable,
        orientation: Transform2,
        depth: u32,
        color_profile: Option<Profile>,
    ) -> CairoObject {
        CairoObject {
            state: Rc::new(RefCell::new(CairoObjectState {
                orientation,
                content: match color_profile.clone() {
                    Some(color_profile) => color_profile.transform_content(content),
                    None => content,
                },
                depth,
                redraw: RefCell::new(true),
            })),
            color_profile,
            cache_surface: Rc::new(RefCell::new(None)),
        }
    }
    fn redraw(&self, pixel_ratio: f64) {
        let state = self.state.borrow();
        let mut redraw = state.redraw.borrow_mut();
        if !*redraw {
            return;
        }
        *redraw = false;
        if let Rasterizable::Path(path) = &state.content {
            if !path.shadows.is_empty() || !path.clip_segments.is_empty() {
                let mut corners = (
                    Vector2::from((std::f64::INFINITY, std::f64::INFINITY)),
                    Vector2::from((0., 0.)),
                );
                for shadow in &path.shadows {
                    let bounds = path.bounds();
                    let size = bounds.size;
                    let new_size = size + ((shadow.spread + shadow.blur) * 2.);
                    let scale_offset = (size - new_size) / 2.;
                    let near_corner = scale_offset + shadow.offset - shadow.blur;
                    let far_corner = near_corner + new_size + shadow.blur;
                    corners.1.x = corners.1.x.max(far_corner.x);
                    corners.1.y = corners.1.y.max(far_corner.y);
                    corners.0.x = corners.0.x.min(near_corner.x);
                    corners.0.y = corners.0.y.min(near_corner.y);
                }
                let bounds = path.bounds();
                corners = (
                    (
                        bounds.position.x.min(corners.0.x),
                        bounds.position.y.min(corners.0.y),
                    )
                        .into(),
                    (
                        (bounds.position + bounds.size).x.max(corners.1.x),
                        (bounds.position + bounds.size).y.max(corners.1.y),
                    )
                        .into(),
                );
                let size = Vector2::from((
                    (corners.1.x - corners.0.x).abs(),
                    (corners.1.y - corners.0.y).abs(),
                )) * pixel_ratio;
                let base_surface =
                    ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
                let base_context = CairoContext(cairo::Context::new(&base_surface));
                for shadow in &path.shadows {
                    let spread = shadow.spread * 2.;
                    let bounds = path.bounds();
                    let size = bounds.size;
                    let scale = (size + spread) / size;
                    let segments = path.segments.iter();
                    let new_size = size + spread;
                    let surface = ImageSurface::create(
                        Format::ARgb32,
                        ((new_size.x + (shadow.blur * 4.)) * pixel_ratio) as i32,
                        ((new_size.y + (shadow.blur * 4.)) * pixel_ratio) as i32,
                    )
                    .unwrap();
                    let context = CairoContext(cairo::Context::new(&surface));
                    let scale_offset = (size - new_size) / 2.;
                    context.scale(pixel_ratio, pixel_ratio);
                    context.translate(shadow.blur * 2., shadow.blur * 2.);
                    context.scale(scale.x, scale.y);
                    context.translate(-bounds.position.x, -bounds.position.y);
                    segments.for_each(|segment| match segment {
                        Segment::LineTo(point) => {
                            context.line_to(point.x, point.y);
                        }
                        Segment::MoveTo(point) => {
                            context.move_to(point.x, point.y);
                        }
                        Segment::CubicTo(point, handle_1, handle_2) => {
                            context.curve_to(
                                handle_1.x, handle_1.y, handle_2.x, handle_2.y, point.x, point.y,
                            );
                        }
                        Segment::QuadraticTo(point, handle) => {
                            context
                                .curve_to(handle.x, handle.y, handle.x, handle.y, point.x, point.y);
                        }
                    });
                    if path.closed {
                        context.close_path();
                    }
                    context.set_source_rgba(
                        f64::from(shadow.color.r) / 255.,
                        f64::from(shadow.color.g) / 255.,
                        f64::from(shadow.color.b) / 255.,
                        f64::from(shadow.color.a) / 255.,
                    );
                    context.fill();
                    let image = CairoImage::new(CairoSurface(surface));
                    if shadow.blur != 0. {
                        image.blur(shadow.blur);
                    }
                    base_context.set_source_surface(
                        &image.0.borrow_mut().0,
                        (scale_offset.x + shadow.offset.x - (shadow.blur * 2.) - corners.0.x
                            + bounds.position.x)
                            * pixel_ratio,
                        (scale_offset.y + shadow.offset.y - (shadow.blur * 2.) - corners.0.y
                            + bounds.position.y)
                            * pixel_ratio,
                    );
                    base_context.paint();
                }
                base_context.scale(pixel_ratio, pixel_ratio);
                let path = path.clone().with_offset(-corners.0);
                draw_path(&base_context, &path, pixel_ratio);
                composite_clip(&base_context, &path);
                *self.cache_surface.borrow_mut() = Some((
                    base_context,
                    Vector2::from((corners.0.x.min(0.), corners.0.y.min(0.))) * pixel_ratio,
                ));
            }
        }
    }
}

impl Object for CairoObject {
    fn get_transform(&self) -> Transform2 {
        self.state.borrow().orientation
    }
    fn apply_transform(&mut self, transform: Transform2) {
        self.state.borrow_mut().orientation.transform(transform);
    }
    fn set_transform(&mut self, transform: Transform2) {
        self.state.borrow_mut().orientation = transform;
    }
    fn update(&mut self, input: Rasterizable) {
        let mut state = self.state.borrow_mut();
        *state.redraw.borrow_mut() = if let Rasterizable::Path(path) = &input {
            if let Rasterizable::Path(current_path) = &self.state.borrow().content {
                current_path.shadows != path.shadows
                    || current_path.segments != path.segments
                    || !(current_path.clip_segments.is_empty()
                        && current_path.clip_segments.is_empty())
            } else {
                false
            }
        } else {
            false
        };
        state.content = match self.color_profile.clone() {
            Some(color_profile) => color_profile.transform_content(input),
            None => input,
        };
    }
    fn get_depth(&self) -> u32 {
        self.state.borrow().depth
    }
    fn set_depth(&mut self, depth: u32) {
        self.state.borrow_mut().depth = depth;
    }
    fn box_clone(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct Cairo {
    state: Rc<RefCell<CairoState>>,
}

fn new_shader(source: &str, kind: GLenum) -> GLuint {
    unsafe {
        let id = gl::CreateShader(kind);
        let source_string = CString::new(source).unwrap();
        gl::ShaderSource(id, 1, &(source_string).as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        id
    }
}

struct CairoState {
    root_frame: Option<Box<dyn Frame>>,
    event_sender: Sender<Event>,
    event_stream: Receiver<Event>,
    event_task: Rc<AtomicTask>,
    size: ObserverCell<Vector2>,
}

#[derive(Clone)]
struct CairoInput {
    event_stream: Receiver<Event>,
    event_task: Rc<AtomicTask>,
}

impl Input for CairoInput {
    fn box_clone(&self) -> Box<dyn Input> {
        Box::new(self.clone())
    }
}

impl Rasterizer for Cairo {
    fn rasterize(&self, input: Rasterizable, size: Vector2) -> Box<dyn ImageRepresentation> {
        //this is probably wrong, just temp
        let mut frame = CairoFrame::new();
        frame.resize(size);
        frame.set_viewport(Rect::new(Vector2::default(), size));
        frame.add(input.into());
        frame.draw();
        frame.surface()
    }
}

impl Stream for CairoInput {
    type Item = Event;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.event_stream.try_recv() {
            Ok(event) => Ok(Async::Ready(Some(event))),
            Err(err) => match err {
                TryRecvError::Disconnected => panic!("Input channel disconnected!"),
                TryRecvError::Empty => {
                    self.event_task.register();
                    Ok(Async::NotReady)
                }
            },
        }
    }
}

impl Provider for Cairo {
    fn input(&self) -> Box<dyn Input> {
        let state = self.state.borrow();
        Box::new(CairoInput {
            event_stream: state.event_stream.clone(),
            event_task: state.event_task.clone(),
        })
    }
}

impl CanvasContext for Cairo {}

impl ActiveCanvas for Cairo {
    fn box_clone(&self) -> Box<dyn ActiveCanvas> {
        Box::new(self.clone())
    }
}

impl InactiveCanvas for Cairo {
    fn run(self: Box<Self>) {
        self.run_with(Box::new(|_| {}));
    }
    fn run_with(self: Box<Self>, mut cb: Box<dyn FnMut(Box<dyn ActiveCanvas>) + 'static>) {
        let (mut el, frame, size, windowed_context) = {
            let state = self.state.borrow();
            let size = state.size.get();
            let size = LogicalSize::new(size.x, size.y);
            let el = glutin::EventsLoop::new();
            let wb = glutin::WindowBuilder::new().with_dimensions(size);
            let windowed_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(wb, &el)
                .unwrap();
            let dpi_factor = windowed_context.get_hidpi_factor();
            let frame = state.root_frame.clone().unwrap();
            frame.set_pixel_ratio(dpi_factor);
            let size = size.to_physical(dpi_factor);
            let size = (size.width, size.height).into();
            (el, frame, size, windowed_context)
        };

        let profile = Profile::from_window(windowed_context.window());

        if let Ok(profile) = profile {
            frame
                .as_any()
                .downcast::<CairoFrame>()
                .unwrap()
                .set_color_profile(profile);
        }

        frame.resize(size);
        frame.set_viewport(Rect::new((0., 0.), size));

        unsafe {
            windowed_context.make_current().unwrap();
            gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        }

        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }

        let mut surface_pointer: *const c_void;

        surface_pointer = frame
            .to_image()
            .as_any()
            .downcast::<CairoImage>()
            .unwrap()
            .get_data_ptr();

        let vert_id = new_shader(
            r#"#version 330 core
layout (location = 0) in vec3 pos;

out vec2 coord;

void main()
{
    gl_Position = vec4(pos, 1.0);
    coord = (pos.xy + vec2(1, 1)) / 2;
}"#,
            gl::VERTEX_SHADER,
        );

        let frag_id = new_shader(
            r#"#version 330 core
out vec4 FragLDRColor;
  
in vec2 coord;

uniform sampler2D tex;

void main()
{
    FragLDRColor = texture(tex, coord);
}"#,
            gl::FRAGMENT_SHADER,
        );

        let program = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vert_id);
            gl::AttachShader(id, frag_id);
            gl::LinkProgram(id);
            id
        };

        let vertices: Vec<f32> = vec![1., -1., 0., 1., 1., 0., -1., -1., 0., -1., 1., 0.];
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null(),
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        let mut running = true;
        let mut last_time = SystemTime::now();
        let send_events = {
            let state = self.state.borrow();
            Rc::strong_count(&state.event_task) != 1
        };
        while running {
            el.poll_events(|event| {
                let state = self.state.borrow();
                let e = if let glutin::Event::WindowEvent { event, .. } = event.clone() {
                    match event {
                        glutin::WindowEvent::CloseRequested => {
                            running = false;
                            None
                        }
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = windowed_context.get_hidpi_factor();
                            let true_size = logical_size.to_physical(dpi_factor);
                            windowed_context.resize(true_size);
                            state.size.set((true_size.width, true_size.height).into());
                            Some(Event::Windowing(WindowingEvent::Resize))
                        }
                        glutin::WindowEvent::Moved(p) => {
                            Some(Event::Windowing(WindowingEvent::Move((p.x, p.y).into())))
                        }
                        glutin::WindowEvent::CursorMoved { position, .. } => Some(Event::Mouse(
                            MouseEvent::Move((position.x, position.y).into()),
                        )),
                        glutin::WindowEvent::MouseInput {
                            state: element_state,
                            button,
                            ..
                        } => Some(Event::Mouse(match element_state {
                            glutin::ElementState::Pressed => MouseEvent::Down(match button {
                                glutin::MouseButton::Left => mouse::Button::Left,
                                glutin::MouseButton::Right => mouse::Button::Right,
                                glutin::MouseButton::Middle => mouse::Button::Middle,
                                glutin::MouseButton::Other(x) => mouse::Button::Auxiliary(x),
                            }),
                            glutin::ElementState::Released => MouseEvent::Up(match button {
                                glutin::MouseButton::Left => mouse::Button::Left,
                                glutin::MouseButton::Right => mouse::Button::Right,
                                glutin::MouseButton::Middle => mouse::Button::Middle,
                                glutin::MouseButton::Other(x) => mouse::Button::Auxiliary(x),
                            }),
                        })),
                        glutin::WindowEvent::MouseWheel { delta, .. } => {
                            let pixel_delta: Vector2 = match delta {
                                glutin::MouseScrollDelta::LineDelta(_x, _y) => {
                                    println!("LineDelta is not handled");
                                    (0., 0.).into()
                                }
                                glutin::MouseScrollDelta::PixelDelta(p) => (p.x, p.y).into(),
                            };
                            Some(Event::Mouse(MouseEvent::Scroll(pixel_delta)))
                        }
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            let key = native::input::keyboard::parse_code(input.scancode);
                            Some(Event::Keyboard(KeyboardEvent {
                                action: match input.state {
                                    glutin::ElementState::Pressed => keyboard::Action::Down(key),
                                    glutin::ElementState::Released => keyboard::Action::Up(key),
                                },
                                // TODO
                                printable: None,
                            }))
                        }
                        _ => None,
                    }
                } else {
                    None
                };
                e.map(|e| {
                    if send_events {
                        state.event_sender.send(e).unwrap();
                        state.event_task.notify()
                    }
                });
            });

            let state = self.state.borrow();

            let now = SystemTime::now();
            if send_events {
                state
                    .event_sender
                    .send(Event::Windowing(WindowingEvent::Redraw(
                        now.duration_since(last_time).unwrap().as_nanos() as f64 / 1_000_000.,
                    )))
                    .unwrap();
                state.event_task.notify();
            }
            last_time = now;

            if state.size.is_dirty() {
                let size = state.size.get();
                frame.set_viewport(Rect::new((0., 0.), size));
                frame.resize(size);
                surface_pointer = frame
                    .to_image()
                    .as_any()
                    .downcast::<CairoImage>()
                    .unwrap()
                    .get_data_ptr();
            }

            frame.draw();

            let size = state.size.get();

            unsafe {
                gl::Viewport(0, 0, size.x as i32, size.y as i32);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    size.x as i32,
                    size.y as i32,
                    0,
                    gl::BGRA,
                    gl::UNSIGNED_BYTE,
                    surface_pointer,
                );
                gl::UseProgram(program);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            }
            windowed_context.swap_buffers().unwrap();
            cb(self.clone());
        }
    }
}

impl InteractiveCanvas for Cairo {
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveCanvas> {
        {
            let mut state = self.state.borrow_mut();
            state.root_frame = Some(root);
        }
        self
    }
}

impl Canvas for Cairo {
    fn frame(&self) -> Box<dyn Frame> {
        CairoFrame::new()
    }
}

pub(crate) fn new() -> Box<dyn InteractiveCanvas> {
    let (event_sender, event_stream) = unbounded();
    let window = Cairo {
        state: Rc::new(RefCell::new(CairoState {
            //need to figure out how to select size, temp default
            size: ObserverCell::new((700., 700.).into()),
            root_frame: None,
            event_task: Rc::new(AtomicTask::new()),
            event_stream,
            event_sender,
        })),
    };

    Box::new(window)
}
