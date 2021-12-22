# Get this version of nightly rust to test shared memory for wasm
rustup component add rust-src --toolchain nightly-2021-07-29-x86_64-pc-windows-msvc

# Install wasm-bindgen-cli to build wasm bindings
cargo install -f wasm-bindgen-cli
