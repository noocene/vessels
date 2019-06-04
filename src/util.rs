use std::sync::RwLock;

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

struct ObserverCellState<T> {
    content: T,
    dirty: bool,
}

pub(crate) struct ObserverCell<T>
where
    T: Copy,
{
    state: RwLock<ObserverCellState<T>>,
}

impl<T> ObserverCell<T>
where
    T: Copy,
{
    pub(crate) fn new(value: T) -> Self {
        ObserverCell {
            state: RwLock::new(ObserverCellState {
                content: value,
                dirty: true,
            }),
        }
    }
    pub(crate) fn is_dirty(&self) -> bool {
        self.state.read().unwrap().dirty
    }
    pub(crate) fn get(&self) -> T {
        {
            self.state.write().unwrap().dirty = false;
        }
        self.state.read().unwrap().content
    }
    pub(crate) fn set(&self, value: T) {
        let mut state = self.state.write().unwrap();
        state.content = value;
        state.dirty = true;
    }
}