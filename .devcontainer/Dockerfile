FROM syntacticsugarglider/vitruvia
RUN apt-get --allow-releaseinfo-change update
RUN apt-get -y install libssl-dev git
RUN cargo install cargo-edit
RUN cargo install cargo-expand
RUN apt-get -y install zsh
RUN rustup component add rls
RUN rustup component add clippy
RUN rustup component add rust-analysis
RUN rustup component add rust-src
RUN rustup component add rustfmt
RUN rustup install nightly