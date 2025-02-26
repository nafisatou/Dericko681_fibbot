FROM rust:latest

WORKDIR /usr/src/fibbot

COPY . .

RUN cargo build --release

ENTRYPOINT ["/usr/src/fibbot/target/release/fibbot"]
