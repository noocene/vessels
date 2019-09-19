use vitruvia::{
    graphics_2d::{self, Color, Transform},
    path::{Primitive, Shadow},
};

fn main() {
    let gfx = graphics_2d::new();
    let mut root = gfx.frame();
    root.add(
        Primitive::square(100.)
            .clip(Primitive::circle(120.).finalize().with_origin((20., 20.)))
            .shadow(
                Shadow::new(Color::black().with_alpha(75))
                    .offset((-10., -10.))
                    .spread(10.)
                    .blur(10.),
            )
            .shadow(
                Shadow::new(Color::black().with_alpha(150))
                    .offset((10., 10.))
                    .spread(10.)
                    .blur(10.),
            )
            .fill(Color::rgba(0, 0, 255, 255).into())
            .finalize()
            .with_origin((10., 10.))
            .into(),
    )
    .apply_transform(
        Transform::default()
            .with_position((50., 50.))
            .with_scale((2., 2.)),
    );
    /*root.add(
        Primitive::circle(30.)
            .fill(Color::black().into())
            .finalize()
            .into(),
    );*/
    gfx.start(root).run();
}
