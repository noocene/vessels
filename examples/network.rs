use vessels::{
    channel::IdChannel,
    core,
    core::{hal::network::Client, Executor},
    format::Cbor,
    log,
};

pub fn main() {
    core::<Executor>().unwrap().run(async move {
        let mut network = Client::new().unwrap();
        let data = network
            .connect::<String, IdChannel, Cbor>("ws://127.0.0.1:61200".parse().unwrap())
            .await
            .unwrap();
        log!("{}", data);
    });
}
