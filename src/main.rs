use valuedev::{
    channel::IdChannel,
    format::{Decode, Encode, Json},
    OnTo,
};

use futures::{Future, Stream};

fn main() {
    tokio::run(
        None::<bool>
            .on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .and_then(|item: Option<bool>| {
                println!("{:?}", item);
                Ok(())
            }),
    )
}
