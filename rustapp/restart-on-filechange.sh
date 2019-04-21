#!/usr/bin/env bash

apt-get update
apt-get install -y inotify-tools

export RUST_BACKTRACE=1

sigint_handler()
{
  kill $PID
  exit
}

trap sigint_handler SIGINT

cargo build --release --target=x86_64-unknown-linux-musl
while true; do
  cargo run --release --target=x86_64-unknown-linux-musl &
  PID=$!
  sleep 5
  inotifywait -e modify -e move -e create -e delete -e attrib -r `pwd`
  kill $PID
done