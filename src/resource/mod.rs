use core::marker::PhantomData;

mod rehydrate;
pub use rehydrate::Rehydrate;
pub mod hash;
use hash::Algorithm;
pub mod manager;
pub use manager::{ErasedResourceManager, ResourceManagerExt};

pub struct Resource<T, U: Rehydrate<T>, A: Algorithm>(A::Hash, PhantomData<(T, U)>);

impl<T, U: Rehydrate<T>, A: Algorithm> Clone for Resource<T, U, A>
where
    A::Hash: Clone,
{
    fn clone(&self) -> Self {
        Resource(self.0.clone(), PhantomData)
    }
}

impl<T, U: Rehydrate<T>, A: Algorithm> Resource<T, U, A> {
    pub fn new(data: A::Hash) -> Self {
        Resource(data, PhantomData)
    }

    pub fn hash(&self) -> A::Hash
    where
        A::Hash: Clone,
    {
        self.0.clone()
    }
}
