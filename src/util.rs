use std::cell::Cell;

trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

pub(crate) struct ObserverCell<T>
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
    pub(crate) fn new(value: T) -> Self {
        ObserverCell {
            cell: Cell::new(value),
            dirty: Cell::new(true),
        }
    }
    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty.get()
    }
    pub(crate) fn get(&self) -> T {
        self.dirty.set(false);
        self.cell.get()
    }
    pub(crate) fn set(&self, value: T) {
        self.cell.set(value);
        self.dirty.set(true);
    }
}
