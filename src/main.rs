mod value;
use futures::{lazy, Future, Stream};
use std::marker::PhantomData;
pub use value::*;
#[macro_use]
extern crate erased_serde;
fn main() {
    tokio::run(25u32.stream::<IdChannel>().map(JSON::format).and_then(|c| {
        c.for_each(|i| {
            println!("{}", i);
            Ok(())
        })
    }));
}
