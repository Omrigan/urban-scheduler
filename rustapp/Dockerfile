FROM rustlang/rust:nightly

WORKDIR /rustapp
COPY . .


RUN cargo build --release

RUN cargo install --path .

CMD ["usrust"]