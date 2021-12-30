FROM ekidd/rust-musl-builder:stable as cargo-build

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

WORKDIR /src/app

COPY .env .

COPY --from=cargo-build /usr/src/app/target/x86_64-unknown-linux-musl/release/telerust .

ENV RUST_LOG=info

# Run the application
CMD ["./telerust"]