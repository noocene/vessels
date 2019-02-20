use crate::graphics::*;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use stdweb::web::event::ResizeEvent;

use stdweb::web::html_element::CanvasElement;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct Canvas {}

impl Graphics for Canvas {
    fn run(&self, _root: Box<Frame<Object<Geometry, Material>>>) {}
}

impl GraphicsEmpty for Canvas {}

impl Graphics2D for Canvas {
    fn frame(&mut self) -> Frame2D {
        Box::new(CanvasFrame {})
    }
}

pub struct CanvasFrame {}

impl Object<dyn Geometry2D, TextureTarget2D> for CanvasFrame {}

impl<'a> Frame<'a, Object2D> for CanvasFrame {
    fn add(&self, _object: &'a Object2D) {}
}

impl crate::util::TryInto<Box<Graphics2D>> for Canvas {
    type Error = ();
    fn try_into(self) -> Result<Box<dyn Graphics2D>, Self::Error> {
        Ok(Box::new(self))
    }
}

pub fn initialize() -> impl GraphicsEmpty {
    stdweb::initialize();

    let d = document();

    let canvas: CanvasElement = d.create_element("canvas").unwrap().try_into().unwrap();

    d.body().unwrap().append_child(&canvas);

    d.head()
        .unwrap()
        .append_html(
            r#"
<style>
body {
    margin: 0;
}
canvas {
    width: 100%;
    height: 100%;
}
</style>
            "#,
        )
        .unwrap();

    let _context: CanvasRenderingContext2d = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    Canvas {}
}
