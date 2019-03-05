use vitruvia::graphics;
use vitruvia::graphics::{
    Distance2D, Frame2D, Graphics2D, StaticObject2D, StrokeBuilder, VectorEntity2DShadow,
    VectorGeometryPrimitive, RGBA8,
};

fn main() {
    let gfx = graphics::new();
    let mut root = gfx.frame();
    root.add(StaticObject2D::from_entity(
        VectorGeometryPrimitive::square(100.)
            .stroke(
                StrokeBuilder::new(RGBA8::black().into(), 2.)
                    .join_round()
                    .finalize(),
            )
            .fill(RGBA8::black().with_alpha(127).into())
            .shadow(
                VectorEntity2DShadow::new(RGBA8::black().with_alpha(50))
                    .blur(10.)
                    .offset(Distance2D::new(5., 5.)),
            )
            .finalize(),
    ));
    gfx.run(root);
}
