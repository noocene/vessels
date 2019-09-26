use vitruvia::graphics::{
    self,
    path::{Primitive, Shadow},
    LDRColor, Transform2,
};

fn main() {
    let gfx = graphics::canvas::new();
    let mut root = gfx.frame();
    root.add(
        Primitive::square(100.)
            //.clip(Primitive::circle(120.).finalize().with_offset((-20., -20.)))
            .shadow(
                Shadow::new(LDRColor::black().with_alpha(75))
                    .offset((-10., -10.))
                    .spread(10.)
                    .blur(10.),
            )
            .shadow(
                Shadow::new(LDRColor::black().with_alpha(150))
                    .offset((10., 10.))
                    .spread(10.)
                    .blur(10.),
            )
            .fill(LDRColor::rgba(0, 0, 255, 255).into())
            .finalize()
            .with_offset((-10., -10.))
            .into(),
    )
    .apply_transform(Transform2::default().with_position((50., 50.)));
    /*root.add(
        Primitive::circle(30.)
            .fill(LDRColor::black().into())
            .finalize()
            .into(),
    );*/
    gfx.start(root).run();
}
