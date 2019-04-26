use crate::graphics_2d::*;
use crate::input::*;
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
use cairo::{Format, ImageSurface, ImageSurfaceData};

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

impl ImageRepresentation for CairoImage {
    fn get_size(&self) -> Vector {
        (
            self.0.lock().unwrap().get_width() as f64,
            self.0.lock().unwrap().get_height() as f64,
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
}

impl Clone for CairoFrame {
    fn clone(&self) -> Self {
        CairoFrame {
            state: self.state.clone(),
        }
    }
}

impl Frame for CairoFrame {
    fn add(&mut self, rasterizable: Rasterizable, orientation: Transform) -> Box<dyn Object> {
        let object = CairoObject::new(rasterizable, orientation);
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
        let size = size.into();
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

    fn measure(&self, input: Text) -> Vector {
        //temporary
        Vector { x: 5.0, y: 5.0 }
    }

    fn box_clone(&self) -> Box<dyn Frame> {
        Box::new(CairoFrame {
            state: self.state.clone(),
        })
    }

    fn show(&self) {
        //show
    }

    fn draw(&self) {
        let state = self.state.read().unwrap();
        let context = state.context.lock().unwrap();
        context.set_source_rgb(1., 1., 1.);
        context.paint();
        context.set_source_rgb(0., 0., 0.);
        context.set_line_width(1.);
        context.rectangle(0., 0., 100., 100.);
        context.stroke_preserve();
        context.fill();
    }
}

struct CairoObjectState {
    orientation: Transform,
    content: Rasterizable,
}

#[derive(Clone)]
struct CairoObject {
    state: Arc<RwLock<CairoObjectState>>,
}

impl CairoObject {
    fn new(content: Rasterizable, orientation: Transform) -> CairoObject {
        CairoObject {
            state: Arc::new(RwLock::new(CairoObjectState {
                orientation,
                content,
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

struct Window {
    state: Arc<RwLock<WindowState>>,
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

struct WindowState {
    root_frame: Option<Box<dyn Frame>>,
    event_handler: EventHandler,
    size: ObserverCell<(f64, f64, f64)>,
}

impl Ticker for Window {
    fn bind(&mut self, handler: Box<dyn FnMut(f64) + 'static + Send + Sync>) {}
}

impl Rasterizer for Window {
    fn rasterize(&self, input: Rasterizable, size: Vector) -> Box<dyn ImageRepresentation> {
        //this is probably wrong, just temp
        let mut frame = CairoFrame::new();
        frame.resize(size);
        frame.set_viewport(Rect::new(Vector::default(), size));
        frame.add(input, Vector::from((0., 0.)).into());
        frame.draw();
        frame.surface()
    }
}

impl Context for Window {
    fn mouse(&self) -> Box<dyn Mouse> {
        native::input::Mouse::new()
    }
    fn keyboard(&self) -> Box<dyn Keyboard> {
        native::input::Keyboard::new()
    }
}

impl ContextGraphics for Window {}

impl InactiveContextGraphics for Window {
    fn run(self: Box<Self>, mut cb: Box<dyn FnMut(Box<dyn ContextGraphics>) + 'static>) {
        let state = self.state.read().unwrap();
        let size = state.size.get();
        state
            .root_frame
            .as_ref()
            .unwrap()
            .resize((size.0, size.1).into());
        let mut el = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new().with_dimensions(LogicalSize::new(size.0, size.1));
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .unwrap();

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
    coord = pos.xy;
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

        state
            .size
            .set((700., 700., windowed_context.get_hidpi_factor()));

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
                                .set((true_size.width, true_size.height, dpi_factor).into());
                        }
                        _ => (),
                    }
                }
            });
            let state = self.state.read().unwrap();

            if state.size.is_dirty() {
                self.state
                    .read()
                    .unwrap()
                    .root_frame
                    .as_ref()
                    .unwrap()
                    .resize((size.0, size.1).into());
                let root_frame = state.root_frame.as_ref().unwrap();
                surface_pointer = root_frame
                    .to_image()
                    .as_any()
                    .downcast::<CairoImage>()
                    .unwrap()
                    .get_data_ptr();
            }

            let size = state.size.get();

            unsafe {
                gl::Viewport(0, 0, size.0 as i32, size.1 as i32);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    (size.0 / size.2) as i32,
                    (size.1 / size.2) as i32,
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

impl ContextualGraphics for Window {
    fn start(self: Box<Self>, root: Box<dyn Frame>) -> Box<dyn InactiveContextGraphics> {
        {
            let mut state = self.state.write().unwrap();
            state.root_frame = Some(root);
        }
        self
    }
}

impl Graphics for Window {
    fn frame(&self) -> Box<dyn Frame> {
        CairoFrame::new()
    }
}

impl Clone for Window {
    fn clone(&self) -> Window {
        Window {
            state: self.state.clone(),
        }
    }
}

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    let window = Window {
        state: Arc::new(RwLock::new(WindowState {
            //need to figure out how to select size, temp default
            size: ObserverCell::new((700.0, 700.0, 2.).into()),
            event_handler: EventHandler::new(),
            root_frame: None,
        })),
    };

    Box::new(window)
}
