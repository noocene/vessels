use kinddev::{
    channel::{Channel, IdChannel},
    format::{Decode, Encode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{Future, Stream};

#[derive(Serialize, Deserialize, Debug, Kind)]
#[kind(using::Serde)]
enum TestEnum {
    Yes(u32),
    No(String),
}

fn main() {
    let meme: [TestEnum; 3] = [
        TestEnum::No("ok this is epic".to_owned()),
        TestEnum::No("ok this is unepic".to_owned()),
        TestEnum::Yes(69),
    ];
    tokio::run(
        meme.on_to::<IdChannel>()
            .map(Json::encode)
            .map(|c| c.inspect(|item| println!("{}", item)))
            .map(Json::decode::<IdChannel>)
            .flatten()
            .and_then(|item: [TestEnum; 3]| {
                println!("{:?}", item);
                Ok(())
            }),
    )
}
