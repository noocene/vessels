pub trait Renderer {
    fn new() -> Self;
    fn run(&self);
    fn root(&self) -> Box<dyn RootFrame>;
}

pub trait Object<T: Geometry> {}

pub enum GeometryBuilder<'a, T: Geometry> {
    Static(&'a T),
    Dynamic(&'a DynamicGeometry<T>),
}

pub trait DynamicGeometry<T: Geometry> {
    fn new() -> DynamicGeometry<T>
    where
        Self: Sized;
    fn on_change(&self, callback: Fn());
    fn render(&self) -> &T;
}

pub trait Geometry {}

pub struct Geometry2D {}

impl Geometry for Geometry2D {}

pub struct Geometry3D {
    pub indices: Vec<u16>,
    pub vertices: Vec<f32>,
}

impl Geometry for Geometry3D {}

pub trait Frame<T: Geometry> {
    fn child(&mut self, bounds: Rect) -> Box<Frame<T>>;
    fn object(&mut self, geo: GeometryBuilder<T>) -> Box<Object<T>>;
}

pub trait Frame2D: Frame<Geometry2D> {}

pub trait Frame3D: Frame<Geometry3D> {}

pub trait RootFrame: Frame2D {}

pub trait ChildFrame<T: Geometry>: Frame<T> {
    fn resize(&mut self, size: Size);
    fn clip(&mut self, start: Option<Point>, end: Option<Point>);
    fn position(&mut self, position: Point);
}

pub trait BufferHandle {}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Size {
    pub w: i32,
    pub h: i32,
}

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
