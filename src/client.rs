use std::net::{Ipv4Addr, SocketAddrV4};

use futures::{Future, Sink};

use vitruvia::network::centralized::socket::{self, ConnectConfig};

static PORT: u16 = 8080;

fn main() {
    let config: ConnectConfig = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT).into();
    let client = socket::connect(config)
        .map_err(|e| eprintln!("connection failed: {:?}", e))
        .and_then(|connection| {
            println!("connected");
            connection
                .send(b"test".to_vec())
                .map_err(|e| eprintln!("send failed: {:?}", e))
                .and_then(|_| Ok(()))
        });

    tokio::run(client);
}
