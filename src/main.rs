use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, Entity2D, Frame2D, Graphics2D, Object2D, Point2D, Rect2D, Scale2D, Size2D,
    StaticObject2D, StrokeCapType, StrokeJoinType, Transform2D, VectorEntity2DSegment,
    VectorEntity2DShadow, VectorEntity2DStroke, VectorEntityTexture, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut frame = gfx.frame();
    frame.add(Object2D::Static(StaticObject2D {
        orientation: Transform2D::default(),
        content: vec![Entity2D {
            orientation: Transform2D::default(),
            closed: false,
            stroke: Some(VectorEntity2DStroke {
                content: VectorEntityTexture::Solid(RGBA8 {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
                cap: StrokeCapType::Round,
                join: StrokeJoinType::Round,
                width: 5.,
            }),
            shadow: Some(VectorEntity2DShadow {
                blur: 10.,
                color: RGBA8 {
                    r: 127,
                    g: 127,
                    b: 127,
                    a: 127,
                },
                offset: Distance2D { x: 0., y: 0. },
            }),
            fill: None,
            segments: vec![
                VectorEntity2DSegment::LineTo(Point2D { x: 0., y: 100. }),
                VectorEntity2DSegment::LineTo(Point2D { x: 100., y: 100. }),
                VectorEntity2DSegment::QuadraticTo(
                    Point2D { x: 100., y: 0. },
                    Point2D { x: 50., y: 50. },
                ),
            ],
        }],
    }));
    frame.resize(Size2D {
        width: 800.,
        height: 1000.,
    });
    frame.set_viewport(Rect2D::new(400., 500., -20., -20.));
    root.add(Object2D::Dynamic(Box::new(frame)));
    gfx.run(root);
}
