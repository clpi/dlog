FROM rust:nightly

RUN USER=root cargo new --bin dlog
WORKDIR /dlog
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/dlog*
RUN cargo build --release

CMD ["./target/release/dlog"]
