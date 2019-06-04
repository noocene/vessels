use std::net::{Ipv4Addr, SocketAddrV4};

use futures::{Future, Sink, Stream};

use vitruvia::{
    executor::run,
    network::centralized::socket::{self, ConnectConfig},
};

static PORT: u16 = 8080;

fn main() {
    let config: ConnectConfig = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT).into();
    let client = socket::connect(config)
        .map_err(|e| eprintln!("connection failed: {:?}", e))
        .and_then(|connection| {
            println!("connected");
            let (send, receive) = connection.split();
            let send = send
                .send(b"test".to_vec())
                .map_err(|e| eprintln!("send failed: {:?}", e))
                .and_then(|_| Ok(()));
            let receive = receive
                .for_each(|message| {
                    println!("{:?}", message);
                    Ok(())
                })
                .map_err(|e| eprintln!("recv failed: {:?}", e))
                .and_then(|_| Ok(()));
            send.join(receive).and_then(|(_, _)| Ok(()))
        });

    run(client);
}
