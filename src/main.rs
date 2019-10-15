mod value;
use futures::{lazy, Stream};
use serde::de::DeserializeSeed;
pub use value::*;
#[macro_use]
extern crate erased_serde;

fn main() {
    tokio::run(lazy(|| {
        let chan: IdChannel = 25u32.on_new();
        let ctx = chan.context();
        chan.map(|c| {
            let json = JSON::serialize(c);
            println!("{}", json);
            json
        })
        .map(move |s| JSON::deserialize(s, ctx.clone()))
        .for_each(|_| Ok(()))
    }));
}
