use std::cell::Cell;

pub trait TryInto<T> {
    type Error;
    fn try_into<'a>(self) -> Result<&'a T, Self::Error>;
}

pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub struct ObserverCell<T>
where
    T: Copy,
{
    cell: Cell<T>,
    dirty: Cell<bool>,
}

impl<T> ObserverCell<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        ObserverCell {
            cell: Cell::new(value),
            dirty: Cell::new(true),
        }
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }
    pub fn get(&self) -> T {
        self.dirty.set(false);
        self.cell.get()
    }
    pub fn set(&self, value: T) {
        self.cell.set(value);
        self.dirty.set(true);
    }
    pub fn into_inner(self) -> T {
        self.cell.into_inner()
    }
}
