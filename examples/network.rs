use vessels::{
    channel::IdChannel,
    core,
    core::{hal::network::Client, Executor},
    replicate::collections::List,
    format::Cbor,
    log,
};

pub fn main() {
    core::<Executor>().unwrap().run(async move {
        let mut network = Client::new().unwrap();
        let mut data = network
            .connect::<Box<dyn List<String>>, IdChannel, Cbor>("ws://127.0.0.1:61200".parse().unwrap())
            .await
            .unwrap();
        data.push("test".to_owned()).await;
        log!("{}", data.len().await);
    });
}
