FROM rust:1.47.0 as cargo-build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update
RUN apt-get install ca-certificates libssl-dev -y && rm -rf /var/lib/apt/lists/*

WORKDIR /src/app

COPY .env .

COPY --from=cargo-build /usr/src/app/target/release/telerust .

ENV RUST_LOG=info

# Run the application
CMD ["./telerust"]