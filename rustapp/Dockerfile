FROM rustlang/rust:nightly as build

WORKDIR /rustapp
COPY . .


RUN cargo build --release

RUN cargo install --path .

CMD ["usrust"]

FROM alpine:latest

COPY --from=build /usr/local/cargo/bin/usrust /usr/local/bin/usrust

CMD ["usrust"]
