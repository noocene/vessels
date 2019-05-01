use crate::graphics_2d::*;
use crate::interaction::*;
use crate::path::*;
use crate::targets::native;
use crate::text::*;
use crate::util::ObserverCell;

use std::any::Any;
use std::ffi::{c_void, CString};
use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock};

use glutin::dpi::LogicalSize;
use glutin::ContextTrait;

use cairo::Status;
use cairo::{Format, ImageSurface, LineCap, LineJoin, Matrix};

use pango::{FontDescription, Layout, LayoutExt};

use gl::types::*;

use cairo_sys;

struct CairoSurface(ImageSurface);

struct CairoContext(cairo::Context);

unsafe impl Send for CairoSurface {}

impl Deref for CairoSurface {
    type Target = ImageSurface;

    fn deref(&self) -> &ImageSurface {
        &self.0
    }
}

unsafe impl Send for CairoContext {}

impl Deref for CairoContext {
    type Target = cairo::Context;

    fn deref(&self) -> &cairo::Context {
        &self.0
    }
}

struct CairoImage(Arc<Mutex<CairoSurface>>);

impl CairoImage {
    fn new(surface: CairoSurface) -> CairoImage {
        CairoImage(Arc::new(Mutex::new(surface)))
    }

    fn get_data_ptr(&self) -> *const c_void {
        let surface = &self.0.lock().unwrap().0;
        unsafe {
            cairo_sys::cairo_surface_flush(surface.to_raw_none());
            match Status::from(cairo_sys::cairo_surface_status(surface.to_raw_none())) {
                Status::Success => (),
                status => panic!("Cairo Surface borrow error!"),
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

impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector {
        (
            f64::from(self.0.lock().unwrap().get_width()),
            f64::from(self.0.lock().unwrap().get_height()),
        )
            .into()
    }

    fn box_clone(&self) -> Box<dyn ImageRepresentation> {
        Box::new(CairoImage(self.0.clone()))
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

    fn from_texture(texture: Image<Color, Texture2D>) -> CairoImage {
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
    context: Mutex<CairoContext>,
    contents: Vec<CairoObject>,
    viewport: Rect,
    size: Vector,
    pixel_ratio: f64,
}

struct CairoFrame {
    state: Arc<RwLock<CairoFrameState>>,
}

impl CairoFrame {
    fn new() -> Box<CairoFrame> {
        let size = Vector::default();
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        Box::new(CairoFrame {
            state: Arc::new(RwLock::new(CairoFrameState {
                context: Mutex::new(CairoContext(cairo::Context::new(&surface))),
                contents: vec![],
                size: size,
                viewport: Rect {
                    size: Vector::default(),
                    position: (0., 0.).into(),
                },
                pixel_ratio: 1.,
            })),
        })
    }
    fn surface(&self) -> Box<CairoImage> {
        self.draw();
        Box::new(CairoImage::new(CairoSurface(
            ImageSurface::from(
                self.state
                    .read()
                    .unwrap()
                    .context
                    .lock()
                    .unwrap()
                    .get_target(),
            )
            .unwrap(),
        )))
    }
    fn draw_text(&self, matrix: [f64; 6], entity: &Text) {
        let state = self.state.read().unwrap();
        let context = state.context.lock().unwrap();
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
        let layout = pangocairo::functions::create_layout(&context).unwrap();
        layout.set_text(&entity.content);
        let mut font = FontDescription::new();
        font.set_size(pixels_to_pango_points(f64::from(entity.size)));
        font.set_family("San Francisco");
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
        let attribute_list = pango::AttrList::new();
        attribute_list.insert(
            pango::Attribute::new_letter_spacing(
                pixels_to_pango_points(entity.letter_spacing)
            )
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
        pangocairo::functions::show_layout(&context, &layout);
    }
    fn draw_path(&self, matrix: [f64; 6], entity: &Path) {
        let state = self.state.read().unwrap();
        let context = state.context.lock().unwrap();
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
        let segments = entity.segments.iter();
        context.move_to(0., 0.);
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
                        /*let canvas_gradient = state.context.create_linear_gradient(
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
                        state.context.set_stroke_style_gradient(&canvas_gradient);*/
                    } /*Texture::Image(image) => {
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
                    }*/
                    /*Texture::RadialGradient(gradient) => {
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
                    }*/
                    _ => {}
                }
                context.set_line_width(f64::from(stroke.width));
                if entity.fill.is_some() {
                    context.stroke_preserve();
                } else {
                    context.stroke();
                }
                if let Texture::Image(_image) = &stroke.content {
                    context.scale(state.pixel_ratio, state.pixel_ratio);
                }
            }
            None => {}
        }
        match &entity.fill {
            Some(fill) => {
                match &fill.content {
                    Texture::Solid(color) => {
                        context.set_source_rgba(
                            color.r as f64 / 255.,
                            color.g as f64 / 255.,
                            color.b as f64 / 255.,
                            color.a as f64 / 255.,
                        );
                    } /*Texture::Image(image) => {
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
                    }*/
                    /*Texture::LinearGradient(gradient) => {
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
                    }*/
                    /*Texture::RadialGradient(gradient) => {
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
                    }*/
                    _ => {}
                }
                context.fill();
                if let Texture::Image(_image) = &fill.content {
                    context.scale(state.pixel_ratio, state.pixel_ratio);
                }
            }
            None => {}
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
        let mut state = self.state.write().unwrap();
        state.pixel_ratio = ratio;
    }

    fn add(&mut self, content: Content) -> Box<dyn Object> {
        let object = CairoObject::new(content.content, content.transform, content.depth);
        let mut state = self.state.write().unwrap();
        state.contents.push(object.clone());
        Box::new(object)
    }

    fn set_viewport(&self, viewport: Rect) {
        let mut state = self.state.write().unwrap();
        state.viewport = viewport;
    }

    fn resize(&self, size: Vector) {
        let mut state = self.state.write().unwrap();
        state.size = size;
        let surface = ImageSurface::create(Format::ARgb32, size.x as i32, size.y as i32).unwrap();
        state.context = Mutex::new(CairoContext(cairo::Context::new(&surface)));
    }

    fn get_size(&self) -> Vector {
        let state = self.state.read().unwrap();
        state.size
    }

    fn to_image(&self) -> Box<dyn ImageRepresentation> {
        self.surface()
    }

    fn measure(&self, input: Rasterizable) -> Vector {
        //temporary
        Vector { x: 5.0, y: 5.0 }
    }

    fn box_clone(&self) -> Box<dyn Frame> {
        Box::new(CairoFrame {
            state: self.state.clone(),
        })
    }

    fn show(&self) {}

    fn draw(&self) {
        let state = self.state.read().unwrap();
        {
            let context = state.context.lock().unwrap();
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
            let object = object.state.read().unwrap();
            let matrix = object.orientation.to_matrix();
            match &object.content {
                Rasterizable::Path(path) => self.draw_path(matrix, &path),
                Rasterizable::Text(input) => self.draw_text(matrix, &input),
            };
        });
    }
}

struct CairoObjectState {
    orientation: Transform,
    content: Rasterizable,
    depth: u32,
}

#[derive(Clone)]
struct CairoObject {
    state: Arc<RwLock<CairoObjectState>>,
}

impl CairoObject {
    fn new(content: Rasterizable, orientation: Transform, depth: u32) -> CairoObject {
        CairoObject {
            state: Arc::new(RwLock::new(CairoObjectState {
                orientation,
                content,
                depth,
            })),
        }
    }
}

impl Object for CairoObject {
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
    fn get_depth(&self) -> u32 {
        self.state.read().unwrap().depth
    }
    fn set_depth(&mut self, depth: u32) {
        self.state.write().unwrap().depth = depth;
    }
}

struct EventHandler {
    handlers: Vec<Box<dyn Fn(glutin::Event) + Send + Sync>>,
}

impl EventHandler {
    fn new() -> EventHandler {
        EventHandler { handlers: vec![] }
    }

    fn bind_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(glutin::Event) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
}

struct Cairo {
    state: Arc<RwLock<CairoState>>,
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
    event_handler: EventHandler,
    size: ObserverCell<Vector>,
}

impl Ticker for Cairo {
    fn bind(&mut self, handler: Box<dyn FnMut(f64) + 'static + Send + Sync>) {}
}

impl Rasterizer for Cairo {
    fn rasterize(&self, input: Rasterizable, size: Vector) -> Box<dyn ImageRepresentation> {
        //this is probably wrong, just temp
        let mut frame = CairoFrame::new();
        frame.resize(size);
        frame.set_viewport(Rect::new(Vector::default(), size));
        frame.add(input.into());
        frame.draw();
        frame.surface()
    }
}

impl Context for Cairo {
    fn mouse(&self) -> Box<dyn Mouse> {
        native::interaction::Mouse::new()
    }
    fn keyboard(&self) -> Box<dyn Keyboard> {
        native::interaction::Keyboard::new()
    }
    fn window(&self) -> Box<dyn Window> {
        native::interaction::Window::new()
    }
}

impl ContextGraphics for Cairo {}

impl InactiveContextGraphics for Cairo {
    fn run(self: Box<Self>, mut cb: Box<dyn FnMut(Box<dyn ContextGraphics>) + 'static>) {
        let state = self.state.read().unwrap();
        let size = state.size.get();
        let size = LogicalSize::new(size.x, size.y);
        let mut el = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new().with_dimensions(size);
        let windowed_context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let dpi_factor = windowed_context.get_hidpi_factor();
        state
            .root_frame
            .as_ref()
            .unwrap()
            .set_pixel_ratio(dpi_factor);
        let size = size.to_physical(dpi_factor);
        let frame = state.root_frame.as_ref().unwrap();

        let size = (size.width, size.height).into();

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

        {
            let root_frame = state.root_frame.as_ref().unwrap();
            surface_pointer = root_frame
                .to_image()
                .as_any()
                .downcast::<CairoImage>()
                .unwrap()
                .get_data_ptr();
        }

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
out vec4 FragColor;
  
in vec2 coord;

uniform sampler2D tex;

void main()
{
    FragColor = texture(tex, coord);
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
        while running {
            el.poll_events(|event| {
                //temporary event handling
                //println!("{:?}", event);
                if let glutin::Event::WindowEvent { event, .. } = event {
                    match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = windowed_context.get_hidpi_factor();
                            let true_size = logical_size.to_physical(dpi_factor);
                            windowed_context.resize(true_size);
                            self.state
                                .read()
                                .unwrap()
                                .size
                                .set((true_size.width, true_size.height).into());
                        }
                        _ => (),
                    }
                }
            });
            let state = self.state.read().unwrap();

            if state.size.is_dirty() {
                let size = state.size.get();
                let root_frame = state.root_frame.as_ref().unwrap();
                root_frame.set_viewport(Rect::new((0., 0.), size));
                root_frame.resize(size);
                surface_pointer = root_frame
                    .to_image()
                    .as_any()
                    .downcast::<CairoImage>()
                    .unwrap()
                    .get_data_ptr();
            }

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
        }
    }
}

impl ContextualGraphics for Cairo {
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveContextGraphics> {
        {
            let mut state = self.state.write().unwrap();
            state.root_frame = Some(root);
        }
        self
    }
}

impl Graphics for Cairo {
    fn frame(&self) -> Box<dyn Frame> {
        CairoFrame::new()
    }
}

impl Clone for Cairo {
    fn clone(&self) -> Cairo {
        Cairo {
            state: self.state.clone(),
        }
    }
}

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    let window = Cairo {
        state: Arc::new(RwLock::new(CairoState {
            //need to figure out how to select size, temp default
            size: ObserverCell::new((700., 700.).into()),
            event_handler: EventHandler::new(),
            root_frame: None,
        })),
    };

    Box::new(window)
}
