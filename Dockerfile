FROM rust:1.47.0 as cargo-build

RUN apt-get update

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:latest

# Copy the .env file, the Postgres ca-certificate and the compiled binary
COPY .env .
COPY --from=cargo-build /usr/src/app/target/release/telerust .

ENV RUST_LOG=info

# Run the application
CMD ["RUST_LOG=info ./telerust"]