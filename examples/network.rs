use vessels::{
    core,
    core::{executor::Spawn, hal::network::Client, Executor},
    log,
};

pub fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let mut network = Client::<String>::new().unwrap();
        let data = network
            .connect("ws://127.0.0.1:61200".parse().unwrap())
            .await
            .unwrap();
        log!("{}", data);
    });
}