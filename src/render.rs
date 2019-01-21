pub trait Renderer {
    fn new() -> Self;
    fn run(&self);
    fn root(&self) -> Box<dyn RootFrame>;
}

pub trait RootFrame {
    fn new(&mut self, bounds: Rect) -> Box<dyn Frame>;
}

pub trait Frame {
    fn resize(&mut self, size: Size);
    fn clip(&mut self, start: Option<Point>, end: Option<Point>);
    fn position(&mut self, position: Point);
    fn new(&mut self, bounds: Rect) -> Box<dyn Frame>;
}

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
