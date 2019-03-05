use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, Entity2D, EntityFormat2D, Frame2D, Graphics2D, Object2D, Orientation2D, Point2D,
    StaticObject2D, StrokeCapType, StrokeJoinType, VectorEntity2D, VectorEntity2DFill,
    VectorEntity2DSegment, VectorEntity2DStroke, VectorEntityColor, VectorEntityGradientStop,
    VectorEntityLinearGradient, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    root.add(Object2D::Static(StaticObject2D {
        orientation: Orientation2D::default(),
        content: vec![Entity2D {
            orientation: Orientation2D::default(),
            representation: EntityFormat2D::VectorEntity2D(VectorEntity2D {
                closed: true,
                stroke: Some(VectorEntity2DStroke {
                    color: VectorEntityColor::Solid(RGBA8 {
                        r: 127,
                        g: 127,
                        b: 127,
                        a: 255,
                    }),
                    cap: StrokeCapType::Round,
                    join: StrokeJoinType::Round,
                    width: 10,
                }),
                fill: Some(VectorEntity2DFill {
                    color: VectorEntityColor::LinearGradient(VectorEntityLinearGradient {
                        start: Point2D { x: 100., y: 100. },
                        end: Point2D { x: 150., y: 150. },
                        stops: vec![
                            VectorEntityGradientStop {
                                color: RGBA8 {
                                    r: 255,
                                    g: 0,
                                    b: 0,
                                    a: 255,
                                },
                                offset: 0.,
                            },
                            VectorEntityGradientStop {
                                color: RGBA8 {
                                    r: 0,
                                    g: 255,
                                    b: 0,
                                    a: 255,
                                },
                                offset: 1.,
                            },
                        ],
                    }),
                }),
                segments: vec![
                    VectorEntity2DSegment::Point(Point2D { x: 50., y: 50. }),
                    VectorEntity2DSegment::Point(Point2D { x: 50., y: 150. }),
                    VectorEntity2DSegment::Point(Point2D { x: 150., y: 150. }),
                    VectorEntity2DSegment::Point(Point2D { x: 150., y: 50. }),
                ],
            }),
        }],
    }));
    gfx.run(root);
}
