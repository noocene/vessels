use vessels::{
    core,
    core::{executor::Spawn, hal::network::Server, Executor},
    log,
};

use futures::StreamExt;

pub fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let mut network = Server::new().unwrap();
        while let Some(_) = network
            .listen("127.0.0.1:61200".parse().unwrap())
            .next()
            .await
        {
            log!("got peer");
        }
    });
}
