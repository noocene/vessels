pub trait Renderer {
    fn new() -> Self;
    fn run(&self);
}

pub trait Pane {}
