use valuedev::{
    channel::IdChannel,
    format::{Decode, Encode, Json},
    value, OnTo,
};

use futures::{future::ok, Future, Stream};

fn main() {
    tokio::run(
        (Box::new(ok(true)) as Box<dyn Future<Item = bool, Error = ()> + Send>)
            .on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .and_then(|item: value::Future<bool, ()>| {
                item.and_then(|item| {
                    println!("item: {}", item);
                    Ok(())
                })
                .map_err(|_| println!("err"))
            }),
    )
}
