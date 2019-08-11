FROM clux/muslrust as build

#RUN apt-get update
#
#RUN apt-get install musl-tools libssl-dev pkg-config -y
#
#RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/usrust

ARG HERE_APP_CODE

ARG HERE_APP_ID

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

RUN export RUSTFLAGS=-Clinker=musl-gcc

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -f target/x86_64-unknown-linux-musl/release/deps/usrust*

COPY src src

#RUN cargo test --release --target=x86_64-unknown-linux-musl

RUN cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

RUN apt-get update  && \
    apt-get install -y liblapack-dev gcc software-properties-common

RUN add-apt-repository ppa:jonathonf/gcc-7.3 # Thanks ubuntu 16.04
RUN apt-get update && apt-get install -y libgfortran4


COPY src/opt/scip /usr/local/bin/scip

COPY src/opt/zimpl /usr/local/bin/zimpl

RUN cp /usr/src/usrust/target/x86_64-unknown-linux-musl/release/usrust /usr/local/bin/usrust

FROM debian:latest

COPY --from=build /usr/local/bin /usr/local/bin

RUN apt-get update  && \
    apt-get install -y liblapack-dev gcc libgfortran4


CMD ["usrust"]
