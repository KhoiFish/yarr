#!/bin/sh
set -ex
cargo build --release
npm --prefix ./web run build