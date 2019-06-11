use futures::{Future, Stream};
use vitruvia::{executor::run, network::mesh::Peer};

fn main() {
    run(Peer::new()
        .and_then(|(peer, negotiation)| {
            negotiation.for_each(|item| {
                println!("{:?}", item);
                Ok(())
            })
        })
        .map_err(|_| ()));
}
