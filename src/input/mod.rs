pub mod keyboard;
pub use keyboard::Keyboard;
pub mod mouse;
pub use mouse::Mouse;

pub trait Source<T>
where
    T: Event,
{
    fn bind<F>(&self, handler: F)
    where
        F: Fn(T) + 'static;
}

pub trait Event {}

pub trait Context {
    type Mouse: Mouse;
    type Keyboard: Keyboard;
    fn mouse(&self) -> Self::Mouse;
    fn keyboard(&self) -> Self::Keyboard;
}
