use vessels::{
    channel::IdChannel,
    core::{hal::network::Client, run},
    format::Cbor,
    log,
    replicate::collections::List,
};

pub fn main() {
    run(async move {
        let mut network = Client::new().unwrap();
        let mut data = network
            .connect::<Box<dyn List<String>>, IdChannel, Cbor>(
                "ws://127.0.0.1:61200".parse().unwrap(),
            )
            .await
            .unwrap();
        data.push("test".to_owned()).await;
        log!("{}", data.len().await);
    });
}
