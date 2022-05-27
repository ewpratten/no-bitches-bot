#! /bin/bash
set -ex

rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release 

docker build -t no-bitches-bot .
docker tag no-bitches-bot ewpratten/no-bitches-bot