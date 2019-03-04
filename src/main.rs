use vitruvia::graphics;
use vitruvia::graphics::{
    ConcreteObject2D, Distance2D, Entity2D, EntityFormat2D, Frame2D, Graphics2D, Point2D,
    VectorEntity2D, VectorEntity2DSegment, VectorEntity2DStroke, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    root.add(Box::new(ConcreteObject2D {
        position: Point2D::default(),
        contents: vec![Entity2D {
            offset: Distance2D::default(),
            representation: EntityFormat2D::VectorEntity2D(VectorEntity2D {
                closed: false,
                stroke: Some(VectorEntity2DStroke {
                    color: RGBA8 {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                    width: 10,
                }),
                fill: None,
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
