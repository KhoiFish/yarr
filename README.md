# YARR - Native & Web-based Raytracer
<img src="yarr_preview.png" alt="YARR - Yet Another Rust Raytracer" />

### Pirate speak for ___"Yet Another Rust Raytracer"___

Ahoy, me fellow mateys! This project serves as my own personal journey with learning Rust. It also led me to cross-compiling to WebAssembly.

Building a raytracer is a great way to learn a modern language like Rust -- building a raytracer can hone your skills in software design and performance-minded implementation. Rust is also well suited for systems programming, where close-to-the-metal control, performance, and data safety are crucial to the craft.

I'm a C/C++ programmer at heart, yet my heart grows ever so fonder for Rust. Rust definitely gives C++ a run for its money.

- Based on Peter Shirley's great "Raytracing in One Weekend" book series
- Core implementation in Rust, with some JavaScript glue
- Builds natively for Windows, MacOS, Linux & Dub-dub-dub (JavaScript & WebAssembly)
- Multi-threaded via rayon crate, wasm-bind-rayon, and as well as a (manually) managed web worker pool
- Tested on iOS/Android via Chrome browser

## Get the source
```console
git clone https://github.com/KhoiFish/yarr.git
```

## Software pre-requisites

To build this project, you'll need to install the following software. *_Note: the following commands are assumed to be from the root directory of the project._*

#### Install rust
https://www.rust-lang.org/tools/install

#### Install npm
https://nodejs.org/en/download/

#### Install this version of nightly Rust
This is required to build multi-thread support in the wasm module:

For Windows
```console
rustup component add rust-src --toolchain nightly-2021-07-29-x86_64-pc-windows-msvc
```

For MacOS
```console
rustup component add rust-src --toolchain nightly-2021-07-29-x86_64-apple-darwin
```

For Linux, do something similar to the above (pay attention to an error in your log if you attempt to build, it should suggest that right toolchain to do rustup with).

#### Install wasm-bindgen-cli to build wasm bindings:
```console
cargo install -f wasm-bindgen-cli
```

#### Install npm packages and dependencies for the web target:
```console
cd web
npm install
```

## Building
You can use the scripts provided in this repo to build and host a local web server, or use these commands directly:

#### Build a native console app
```console
cargo build --release
```

#### Testing native console app (output is a .ppm file, use GIMP to view)
```console
cargo run --release > output.ppm
```

#### Building the web app
```console
npm --prefix ./web run build
```

#### Hosting local web server
```console
npm --prefix ./web run host
```

## References
#### Learn Rust
I recently joined the _Rust Gang_ by reading [Programming Rust: Fast, Safe Systems Development](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590). It introduces and deep-dives the Rust language from the perspective of systems development. It's also surprisingly fun to read with light yet professional humor. If you are coming from another systems language such as C/C++, this is the book for you.

#### Learn Raytracing
If you're new to raytracing, I highly recommend you take a weekend (or two!) to go through Peter Shirley's [RayTracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) series of books. It's a great read and this project's raytracing core is based on it.

#### WebAssembly & Multi-threading on the Web
The following are great resources in learning how to setup Rust & WebAssembly for the web.

* [WASM bindings with wasm-bindgend](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html)
* [Parallel demo with wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/tree/main/examples/raytrace-parallel)
* [Drop-in wasm parallelism with wasm-bindgen-rayon](https://github.com/GoogleChromeLabs/wasm-bindgen-rayon)
* [Manage web worker communication with Comlink](https://github.com/GoogleChromeLabs/comlink)

## Special Thanks
I'd like to thank the following people:
* Peter Shirley, whose books really inspired me. As a kid, I always loved computer generated graphics, and his down-to-earth lessons made my childhood dreams of writing my own raytracer come true.
* Diep Mai, for his friendship, and for introducing me to Rust. I finally got around to learning how-to-rust!

## License
This is free and open-source software distributed under the MIT License. Feel free to use and go get 'em tiger.
