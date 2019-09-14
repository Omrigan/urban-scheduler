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

while true; do
  cargo build --target=x86_64-unknown-linux-musl
  cargo run --target=x86_64-unknown-linux-musl &
  PID=$!
  sleep 3
  inotifywait -e modify -e move -e create -e delete -e attrib -r `pwd`
  sleep 3
  kill $PID
done