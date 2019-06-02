use futures::{Future, Stream};

use vitruvia::network::{
    centralized::socket::{self, Server},
    Connection,
};

static PORT: u16 = 8080;

fn main() {
    let server = socket::listen(PORT)
        .map_err(|e| eprintln!("listen failed: {:?}", e))
        .and_then(|server| {
            server
                .map_err(|e| eprintln!("connect failed: {:?}", e))
                .for_each(|connection| {
                    connection.on_open().and_then(|connection| {
                        println!("connected");
                        connection.on_close().and_then(|connection| {
                            println!("disconnected");
                            Ok(())
                        })
                    })
                })
        });

    tokio::run(server);
}
