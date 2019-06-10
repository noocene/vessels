use futures::{Stream, Future};
use vitruvia::{
    executor::run,
    network::mesh::{Peer, Role},
};

fn main() {
    let (peer, negotiation) = Peer::new(Role::Offering);
    let (peer0, negotiation0) = Peer::new(Role::Answering);
    let (i, o) = negotiation.split();
    let (i0, o0) = negotiation0.split();
    let future = o.forward(i0).join(o0.forward(i));
    run(future.map_err(|_| ()).and_then(|(_, _)| Ok(())));
}
