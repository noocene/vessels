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
    tokio::run(
        vec![
            TestEnum::No("ok this is epic".to_owned()),
            TestEnum::No("ok this is epic".to_owned()),
        ]
        .on_to::<IdChannel>()
        .map(Json::encode)
        .map(|c| c.inspect(|item| println!("{}", item)))
        .map(Json::decode::<IdChannel>)
        .flatten()
        .and_then(|item: Vec<TestEnum>| {
            println!("{:?}", item);
            Ok(())
        }),
    )
}
