use crate::render::Renderer;

use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, IHtmlElement, INode, IElement};

use stdweb::unstable::TryInto;

mod webgl_rendering_context;

use crate::targets::web::webgl_rendering_context::WebGLRenderingContext as gl;

pub struct WebGL {}

impl Renderer for WebGL {
    fn new() -> WebGL {
        stdweb::initialize();
        let doc = document();
        doc.head().unwrap().append_html(r#"
        <style>
        canvas {
            height: 100vh;
            width: 100vw;
            display: block;
        }
        body {
            margin: 0;
        }
        body, html {
            width: 100%;
            height: 100%;
        }
        </style>
        "#).unwrap();
        let canvas: CanvasElement = doc.create_element("canvas").unwrap().try_into().unwrap();
        let context: gl = canvas.get_context().unwrap();
        let body = doc.body().unwrap();
        body.append_child(&canvas);
        WebGL {}
    }
}
