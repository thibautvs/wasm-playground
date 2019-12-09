# wasm-playground
Experimenting with WebAssembly and Rust.

Based on the tutorial from https://opensource.com/article/19/3/calling-rust-javascript.

## Build

* (Install wasm-pack: `cargo install wasm-pack`)
* Package for ES5 JS: `wasm-pack build --target no-modules`

## Run

* (Install Basic HTTP Server: `cargo install basic-http-server`)
* Serve: `basic-http-server`
* Browse: http://127.0.0.1:4000
