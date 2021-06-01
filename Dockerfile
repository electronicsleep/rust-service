FROM rust:1.43.0 AS build
WORKDIR /usr/src/
RUN apt update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new rust_service
WORKDIR /usr/src/rust_service
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /app/
COPY --from=build /usr/local/cargo/bin/rust_service /app/
EXPOSE 8080
USER 1000
ENTRYPOINT ["./rust_service"]
