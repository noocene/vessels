use vessels::{
    channel::IdChannel,
    core::{hal::network::Server, run},
    format::Cbor,
};

pub fn main() {
    run(async move {
        Server::new()
            .unwrap()
            .listen::<String, IdChannel, Cbor>(
                "127.0.0.1:61200".parse().unwrap(),
                Box::new(move || Box::pin(async move { "format".to_string() })),
            )
            .await
            .unwrap();
    });
}
