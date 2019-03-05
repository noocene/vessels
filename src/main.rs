use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, DynamicObject2D, Entity2D, Frame2D, Graphics2D, ImageRepresentation, Object2D,
    Point2D, Rect2D, Size2D, StaticObject2D, StrokeCapType, StrokeJoinType, Transform2D,
    VectorEntity2DFill, VectorEntity2DSegment, VectorEntity2DShadow, VectorEntity2DStroke,
    VectorEntityTexture, RGBA8,
};

use std::borrow::Cow;

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    let mut frame = gfx.frame();

    pub struct TestObject<T>
    where
        T: ImageRepresentation,
    {
        orientation: Transform2D,
        frame: Box<Frame2D<T>>,
    }

    impl<T> TestObject<T>
    where
        T: ImageRepresentation,
    {
        fn new(frame: Box<Frame2D<T>>) -> TestObject<T> {
            let mut frame = frame;
            frame.add(Object2D::Static(StaticObject2D {
                orientation: Transform2D::default(),
                content: vec![Entity2D {
                    orientation: Transform2D::default(),
                    closed: true,
                    stroke: None,
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
                    fill: Some(VectorEntity2DFill {
                        content: VectorEntityTexture::Solid(RGBA8 {
                            r: 0,
                            g: 0,
                            b: 0,
                            a: 255,
                        }),
                    }),
                    segments: vec![
                        VectorEntity2DSegment::LineTo(Point2D { x: 0., y: 100. }),
                        VectorEntity2DSegment::LineTo(Point2D { x: 100., y: 100. }),
                        VectorEntity2DSegment::LineTo(Point2D { x: 100., y: 0. }),
                    ],
                }],
            }));
            frame.resize(Size2D {
                width: 100.,
                height: 100.,
            });
            frame.set_viewport(Rect2D::new(100., 100., 0., 0.));
            TestObject {
                frame,
                orientation: Transform2D::default(),
            }
        }
    }

    impl<T> DynamicObject2D<T> for TestObject<T>
    where
        T: ImageRepresentation,
    {
        fn orientation(&self) -> Transform2D {
            self.orientation.clone()
        }
        fn render(&self) -> Cow<[Entity2D<T>]> {
            Cow::from(vec![Entity2D {
                orientation: Transform2D::default(),
                closed: true,
                stroke: None,
                shadow: None,
                fill: Some(VectorEntity2DFill {
                    content: VectorEntityTexture::Image(self.frame.to_image()),
                }),
                segments: vec![
                    VectorEntity2DSegment::LineTo(Point2D { x: 0., y: 100. }),
                    VectorEntity2DSegment::LineTo(Point2D { x: 100., y: 100. }),
                    VectorEntity2DSegment::LineTo(Point2D { x: 90., y: 0. }),
                ],
            }])
        }
    }
    root.add(Object2D::Dynamic(Box::new(TestObject::new(Box::new(
        frame,
    )))));
    gfx.run(root);
}
