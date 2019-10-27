use kinddev::{
    channel::{Channel, IdChannel},
    format::{Decode, Encode, Json},
    kind::{using, AsKind},
    Kind, OnTo,
};

use serde::{Deserialize, Serialize};

use futures::{Future, Stream};

#[derive(Serialize, Deserialize, Debug)]
enum TestEnum {
    Yes(u32),
    No(String),
}

#[derive(Serialize, Deserialize, Debug, Kind)]
#[kind(using::Serde)]
struct Test {
    e: u32,
    st: Option<String>,
    other: TestEnum,
}

fn main() {
    tokio::run(
        Test {
            e: 20,
            st: Some("test".to_owned()),
            other: TestEnum::Yes(500),
        }
        .on_to::<IdChannel>()
        .map(Json::encode)
        .map(|c| c.inspect(|item| println!("{}", item)))
        .map(Json::decode::<IdChannel>)
        .flatten()
        .and_then(|item: Test| {
            println!("{:?}", item);
            Ok(())
        }),
    )
}
