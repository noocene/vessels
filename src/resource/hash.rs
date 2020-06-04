use core::convert::TryInto;

pub trait Algorithm {
    type Hash;
}

pub trait Hasher<A: Algorithm> {
    fn new() -> Self
    where
        Self: Sized;
    fn write(&mut self, data: &[u8]);
    fn hash(&self) -> A::Hash;
}

pub trait HasherExt<A: Algorithm>: Hasher<A> {
    fn hash<T: Into<Vec<u8>>>(item: T) -> A::Hash
    where
        Self: Sized,
    {
        let mut hasher = Self::new();
        hasher.write(&item.into());
        hasher.hash()
    }

    fn try_hash<T: TryInto<Vec<u8>>>(item: T) -> Result<A::Hash, T::Error>
    where
        Self: Sized,
    {
        let mut hasher = Self::new();
        hasher.write(&item.try_into()?);
        Ok(hasher.hash())
    }
}

impl<A: Algorithm, T: Hasher<A>> HasherExt<A> for T {}
