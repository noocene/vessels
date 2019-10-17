use valuedev::{
    channel::IdChannel,
    format::{Decode, Encode, Json},
    value, OnTo,
};

use futures::{future::ok, Future, Stream};

fn main() {
    tokio::run(
        (Box::new(ok("test".to_owned())) as Box<dyn Future<Item = String, Error = ()> + Send>)
            .on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .and_then(|fut: value::Future<String, ()>| {
                fut.and_then(|item: String| {
                    println!("{}", item);
                    Ok(())
                })
            }),
    )
}
