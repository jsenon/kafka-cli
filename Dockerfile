FROM rust:latest as build
RUN apt-get update && apt-get install -y build-essential libclang-dev clang zlib1g-dev libssl-dev libsasl2-dev
RUN USER=root cargo new --bin kafka-cli
WORKDIR /kafka-cli
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm -rf ./target/release/deps/kafka_cli*
RUN cargo build --release

FROM rust:latest
COPY --from=build /kafka-cli/target/release/kafka-cli .
CMD ["./kafka-cli"]