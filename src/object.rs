pub trait Protocol {
    type Shim;
}

pub struct Object<T: Protocol + ?Sized>(Box<T>);
