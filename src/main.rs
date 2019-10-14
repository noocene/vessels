mod value;
use futures::{lazy, Stream};
pub use value::*;
#[macro_use]
extern crate erased_serde;

fn main() {
    tokio::run(lazy(|| {
        let c = 25u32.on_to::<IdChannel>();
        c.map(|c| {
            let json = serde_json::to_string(&c).unwrap();
            println!("{}", json);
            json
        })
        .for_each(|_| Ok(()))
    }));
}
