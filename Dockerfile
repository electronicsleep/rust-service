FROM rust:1.54.0 AS build
WORKDIR /usr/src/
RUN apt-get update && apt-get install -y musl-tools libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-gnu

RUN USER=root cargo new rust_service
WORKDIR /usr/src/rust_service
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-gnu --path .

FROM debian
WORKDIR /app/
COPY --from=build /usr/local/cargo/bin/rust_service /app/
EXPOSE 8080
USER 1000
ENTRYPOINT ["./rust_service"]
