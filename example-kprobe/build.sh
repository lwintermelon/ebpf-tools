#! /usr/bin/sh
RUSTFLAGS=-g cargo +nightly build --release -Z build-std=core --target=bpfel-unknown-none --package=example-kprobe --bin=example-kern --features=kern --no-default-features
sed -i "s/ty__/type/g"  ../target/bpfel-unknown-none/release/example-kern
cargo build --package=example-kprobe --bin=example-user --features=user --no-default-features