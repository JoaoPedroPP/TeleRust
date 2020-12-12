FROM rust:1.47.0 as cargo-build

# RUN apt-get update

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# FROM ubuntu:latest
FROM alpine:latest

# RUN apt-get update

# Copy the .env file, the Postgres ca-certificate and the compiled binary
WORKDIR /src/app

COPY .env .

COPY --from=cargo-build /usr/src/app/target/release/telerust .

ENV RUST_LOG=info
# RUN  export RUST_LOG=info

# Run the application
CMD ["RUST_LOG=info ./telerust"]