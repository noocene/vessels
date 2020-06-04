use core::convert::Infallible;
use futures::executor::block_on;
use vessels::{acquire, register, with_core, Core};

#[derive(Debug)]
pub struct Tester(String);

fn print() {
    block_on(async move {
        println!("{:?}", acquire::<Tester>().await);
    });
}

fn main() {
    print();

    let core = Core::new();

    with_core! { &core => {
        block_on(async move {
            register(|| async { Ok::<_, Infallible>(Tester("hello there".to_owned())) }).await.unwrap();
        });
    }};

    print();

    with_core! { &core => { call() }};
}

fn call() {
    print();

    let core = Core::new();

    with_core! { &core => { print() }};

    print();
}
