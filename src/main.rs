use futures::{Future, Stream};
use vitruvia::{
    executor::run,
    network::mesh::{Peer, Role},
};

#[macro_use]
extern crate stdweb;

fn main() {
    let (mut peer, negotiation) = Peer::new(Role::Offering);
    let (peer0, negotiation0) = Peer::new(Role::Answering);
    let (i, o) = negotiation.split();
    let (i0, o0) = negotiation0.split();
    let future = o.forward(i0).join(o0.forward(i)).join(peer0.for_each(|_| {console!(log, "data channel"); Ok(())}));
    run(future.map_err(|_| ()).and_then(|(_, _)| Ok(())));
}
