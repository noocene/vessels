use vessels::{
    core,
    core::{executor::Spawn, hal::network::Server, Executor},
    kind::Future,
};

use std::pin::Pin;

use futures::StreamExt;

pub fn main() {
    core::<dyn Executor>().unwrap().run(async move {
        let mut server = Server::new().unwrap();
        server
            .listen(
                "127.0.0.1:61200".parse().unwrap(),
                Box::new(|| Box::pin(async { "hello".to_string() })),
            )
            .await
            .unwrap();
    });
}
