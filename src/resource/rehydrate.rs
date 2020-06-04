use core::future::Future;

pub trait Rehydrate<T>: Sized {
    type RehydrateError;
    type Rehydrate: Future<Output = Result<T, Self::RehydrateError>>;
    type DumpError;
    type Dump: Future<Output = Result<Vec<u8>, Self::DumpError>>;

    fn rehydrate(data: Vec<u8>) -> Self::Rehydrate;
    fn dump(data: T) -> Self::Dump;
}
