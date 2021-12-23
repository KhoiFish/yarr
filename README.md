# Install rust
https://www.rust-lang.org/tools/install

# Install this version of nightly rust to test shared memory for wasm:
rustup component add rust-src --toolchain nightly-2021-07-29-x86_64-pc-windows-msvc

# Install wasm-bindgen-cli to build wasm bindings:
cargo install -f wasm-bindgen-cli

# Install npm
https://nodejs.org/en/download/

# Install node-js packages and dependencies:
cd web
npm install


All intermediates and binaries go into 'target' under the root dir

# Build native/host console app, from root dir:
cargo build --release

# Build web app, from root dir:
npm --prefix ./web run build