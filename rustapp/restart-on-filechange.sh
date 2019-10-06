#!/usr/bin/env bash

apt-get update
#apt-get install -y inotify-tools

export RUST_BACKTRACE=1

sigint_handler()
{
  kill $PID
  exit
}

cargo run --release --target=x86_64-unknown-linux-musl 

trap sigint_handler SIGINT

while true; do
  cargo build --release --target=x86_64-unknown-linux-musl
  cargo run --release --target=x86_64-unknown-linux-musl &
  PID=$!
  sleep 3
  inotifywait -e modify -e move -e create -e delete -e attrib -r `pwd`
  kill $PID
done
