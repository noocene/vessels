use futures::{Future, Sink, Stream};
use vitruvia::{executor::run, network::mesh::Peer};

fn main() {
    run(Peer::new()
        .join(Peer::new())
        .map_err(|err| {
            eprintln!("{:?}", err);
            ()
        })
        .and_then(|((peer, negotiation), (peer0, negotiation0))| {
            let (i, o) = negotiation.split();
            let (i0, o0) = negotiation0.split();

            let mut peer0 = peer0;
            //peer0.data_channel();

            peer.for_each(|channel| {
                println!("channel opened");
                Ok(())
            })
            .join(o.forward(i0).join(o0.forward(i)))
            .map_err(|err| {
                eprintln!("{:?}", err);
                ()
            })
            .and_then(|(_, _)| Ok(()))
        }));
}
