use valuedev::{
    channel::IdChannel,
    format::{Decode, Encode, Json},
    OnTo,
};

use futures::{Future, Stream};

fn main() {
    tokio::run(
        "test"
            .to_owned()
            .on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .and_then(|item: String| {
                println!("{}", item);
                Ok(())
            }),
    )
}
