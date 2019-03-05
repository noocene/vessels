use crate::graphics::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d, FillRule, LineCap, LineJoin};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

use std::cell::RefCell;
use std::rc::Rc;

use std::slice::Iter;

use std::ops::Deref;

type CanvasImage = CanvasElement;

impl ImageRepresentation for CanvasImage {}

impl From<Image<RGBA8, Texture2D>> for CanvasImage {
    fn from(input: Image<RGBA8, Texture2D>) -> CanvasImage {
        let canvas: CanvasElement = document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(input.shape.width);
        canvas.set_height(input.shape.height);
        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let image = context
            .create_image_data(f64::from(input.shape.width), f64::from(input.shape.height))
            .unwrap();
        context.put_image_data(image, 0., 0.).unwrap();
        canvas
    }
}

impl Into<Image<RGBA8, Texture2D>> for CanvasImage {
    fn into(self) -> Image<RGBA8, Texture2D> {
        Image {
            pixels: vec![],
            shape: Texture2D {
                height: 0,
                width: 0,
            },
        }
    }
}

struct CanvasFrame {
    context: CanvasRenderingContext2d,
    canvas: CanvasElement,
    contents: Vec<Object2D<CanvasImage>>,
    pixel_ratio: f64,
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
        }
    }
    fn show(&self) {
        document().body().unwrap().append_child(&self.canvas);
    }
    fn draw(&self) {
        self.contents.iter().for_each(|object| {
            let draw = |base_position: Point2D, content: Iter<Entity2D<CanvasImage>>| {
                content.for_each(|entity| match &entity.representation {
                EntityFormat2D::RasterEntity2D(representation) => {
                    js! {
                        @{&self.context}.drawImage(@{representation.texture.deref()}, @{base_position.x + entity.orientation.position.x}, @{base_position.y + entity.orientation.position.y});
                    }
                }
                EntityFormat2D::VectorEntity2D(representation) => {
                    self.context.begin_path();
                    let segments = representation.segments.iter().enumerate();
                    segments.for_each(|segment| {
                        if let VectorEntity2DSegment::Point(point) = segment.1 {
                            match segment.0 {
                                0 => {
                                    self.context.move_to(
                                        (point.x + entity.orientation.position.x) * self.pixel_ratio,
                                        (point.y + entity.orientation.position.y) * self.pixel_ratio,
                                    );
                                }
                                _ => {
                                    self.context.line_to(
                                        (point.x + entity.orientation.position.x) * self.pixel_ratio,
                                        (point.y + entity.orientation.position.y) * self.pixel_ratio,
                                    );
                                }
                            }
                        }
                    });
                    if representation.closed {
                        self.context.close_path();
                    }
                    match &representation.stroke {
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
                            self.context
                                .set_stroke_style_color(&stroke.color.as_rgba_color());
                            self.context.set_line_width(stroke.width.into());
                            self.context.stroke();
                        }
                        None => {}
                    }
                    match &representation.fill {
                        Some(fill) => {
                            self.context
                                .set_fill_style_color(&fill.color.as_rgba_color());
                            self.context.fill(FillRule::NonZero);
                        }
                        None => {}
                    }
                }
            });
            };
            let base_position: Point2D;
            let content: Iter<Entity2D<CanvasImage>>;
            match object {
                Object2D::Dynamic(object) => {
                    base_position = object.orientation().position;
                    let _content = object.render();
                    content = _content.iter();
                    draw(base_position, content);
                }
                Object2D::Static(object) => {
                    base_position = object.orientation.position;
                    content = object.content.iter();
                    draw(base_position, content);
                }
            }
        });
    }
}

impl DynamicObject2D<CanvasImage> for CanvasFrame {
    fn orientation(&self) -> Orientation2D {
        Orientation2D::default()
    }
    fn render(&self) -> Cow<[Entity2D<CanvasImage>]> {
        Cow::from(vec![Entity2D {
            representation: EntityFormat2D::RasterEntity2D(RasterEntity2D {
                texture: Box::new(self.canvas.clone()),
            }),
            orientation: Orientation2D::default(),
        }])
    }
}

impl Frame2D<CanvasImage> for CanvasFrame {
    fn add(&mut self, object: Object2D<CanvasImage>) {
        self.contents.push(object);
    }
    fn resize(&self, size: Size2D) {
        self.canvas.set_height(size.height as u32);
        self.canvas.set_width(size.width as u32);
    }
    fn get_size(&self) -> Size2D {
        Size2D {
            width: self.canvas.width().into(),
            height: self.canvas.height().into(),
        }
    }
}

struct Canvas {
    state: Rc<RefCell<CanvasState>>,
}

struct CanvasState {
    root_frame: Option<CanvasFrame>,
    size: ObserverCell<Size2D>,
}

impl Graphics2D for Canvas {
    type Image = CanvasImage;
    type Frame = CanvasFrame;
    fn run(self, root: CanvasFrame) {
        let mut state = self.state.borrow_mut();
        root.show();
        state.root_frame = Some(root);
        let cloned = self.clone();
        window().request_animation_frame(move |delta| {
            cloned.animate(delta);
        });
    }
    fn frame(&self) -> CanvasFrame {
        CanvasFrame::new()
    }
}

impl Canvas {
    fn animate(&self, _delta: f64) {
        let state = self.state.borrow();
        match &state.root_frame {
            Some(frame) => {
                if state.size.is_dirty() {
                    frame.resize(state.size.get());
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

pub fn new() -> impl Graphics2D {
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

    let _window = window();

    let gfx = Canvas {
        state: Rc::new(RefCell::new(CanvasState {
            size: ObserverCell::new(Size2D {
                width: f64::from(body.offset_width()) * _window.device_pixel_ratio(),
                height: f64::from(body.offset_height()) * _window.device_pixel_ratio(),
            }),
            root_frame: None,
        })),
    };

    let gfx_resize = gfx.clone();

    _window.add_event_listener(move |_: ResizeEvent| {
        let window = window();
        let state = gfx_resize.state.borrow();
        let body = document().body().unwrap();
        state.size.set(Size2D {
            width: f64::from(body.offset_width()) * window.device_pixel_ratio(),
            height: f64::from(body.offset_height()) * window.device_pixel_ratio(),
        });
    });

    gfx
}
