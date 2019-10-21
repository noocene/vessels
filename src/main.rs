use valuedev::{
    channel::IdChannel,
    format::{Cbor, Decode, Encode},
    value, OnTo,
};

use futures::{future::ok, Future, Stream};

fn main() {
    tokio::run(
        (Box::new(ok(true)) as Box<dyn Future<Item = bool, Error = ()> + Send>)
            .on_to::<IdChannel>()
            .map(Cbor::encode)
            .map(|c| c.inspect(|item| println!("{:?}", item)))
            .map(Cbor::decode::<IdChannel>)
            .flatten()
            .and_then(|item: value::Future<bool, ()>| {
                item.and_then(|item| {
                    println!("{}", item);
                    Ok(())
                })
            }),
    )
}
