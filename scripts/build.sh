#!/bin/sh
set -ex
cargo build --release --features='progress-ui'
npm --prefix ./web run build