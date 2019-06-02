use futures::{Future, Sink, Stream};

use vitruvia::network::{centralized::socket, DataChannel};

static PORT: u16 = 8080;

fn main() {
    let server = socket::listen(PORT)
        .map_err(|e| eprintln!("listen failed: {:?}", e))
        .and_then(|server| {
            server
                .map_err(|e| eprintln!("connect failed: {:?}", e))
                .for_each(|channel| {
                    println!("connected");
                    let (send, receive) = channel.split();
                    let send = send
                        .send(b"test".to_vec())
                        .map_err(|e| eprintln!("send failed: {:?}", e))
                        .and_then(|_| Ok(()));
                    let receive = receive.for_each(|message|{println!("{:?}", message);Ok(())}).map_err(|e| eprintln!("recv failed: {:?}", e))
                        .and_then(|_| Ok(()));
                    send.join(receive).and_then(|(_,_)| Ok(()))
                })
        });

    tokio::run(server);
}
