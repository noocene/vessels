use vessels::{
    channel::IdChannel,
    core,
    core::{hal::network::Server, Executor},
    format::Cbor,
};

pub fn main() {
    core::<Executor>().unwrap().run(async move {
        let mut server = Server::new().unwrap();
        server
            .listen::<String, IdChannel, Cbor>(
                "127.0.0.1:61200".parse().unwrap(),
                Box::new(|| Box::pin(async { "hello".to_string() })),
            )
            .await
            .unwrap();
    });
}
