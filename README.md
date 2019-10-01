# vessels

Client-side microcontainer framework for building a composable ecosystem of software functionality without application boundaries. Not ready for prime time. Documentation [here](https://noocene.github.io/vessels) for master branch, the most up-to-date development branches are [protocol-objects](https://github.com/noocene/vessels/tree/protocol-objects) and [containers](https://github.com/noocene/vessels/tree/containers) for which documentation is not hosted.

The master tree depends on pango/cairo/pangocairo and other branches also depend on gstreamer 1.14 or later for WebRTC on native. The web target uses `wasm-bindgen` (i.e. with Rust's `wasm32-unknown-unknown` target) and has no external dependencies not managed by cargo.