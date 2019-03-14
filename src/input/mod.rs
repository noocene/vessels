pub mod mouse;

pub trait Source<T>
where
    T: Event,
{
    fn bind<F>(&self, handler: F)
    where
        F: Fn(T) + 'static;
}

pub trait Mouse: Source<mouse::Event> {}

pub trait Event {}

pub trait Context {
    type Mouse: Mouse;
    fn mouse(&self) -> Self::Mouse;
}
