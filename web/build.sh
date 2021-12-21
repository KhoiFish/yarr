#!/bin/sh
set -ex

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
  cargo build --target wasm32-unknown-unknown --release #-Z build-std=std,panic_abort

wasm-bindgen \
  ../target/wasm32-unknown-unknown/release/web.wasm \
  --out-dir ./www \
  --target no-modules