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
                    connection
                        .map_err(|e| eprintln!("malformed message: {:?}", e))
                        .for_each(|message| {
                            println!("{:?}", message);
                            Ok(())
                        })
                })
        });

    tokio::run(server);
}
