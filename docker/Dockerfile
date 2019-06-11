FROM debian:buster
RUN apt-get update -y
RUN apt-get install -y libcairo2-dev libpango1.0-dev libpangocairo-1.0-0 libgstreamer1.0-dev gstreamer1.0-plugins-bad
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-web
