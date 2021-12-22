#!/bin/sh
set -ex

# Shared memory is currently disabled in mobile browsers (such as Chrome on iOS).
# Let's keep these flags here so if and when shared memory is re-enabled for mobile browsers
#RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' cargo build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Invoke cargo build
cargo build --target wasm32-unknown-unknown --release

# Generate wasm bindings
wasm-bindgen \
  ../target/wasm32-unknown-unknown/release/web.wasm \
  --out-dir ./www \
  --target web
