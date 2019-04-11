FROM rustlang/rust:nightly as build

RUN apt-get update

RUN apt-get install musl-tools libssl-dev pkg-config -y

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/usrust

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/usrust*

COPY . .

RUN cargo test

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=build /usr/src/usrust/target/x86_64-unknown-linux-musl/release/usrust /usr/local/bin/usrust

CMD ["usrust"]
