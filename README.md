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

## Resources

* https://webassembly.org/
* https://www.rust-lang.org/
* https://developer.mozilla.org/en-US/docs/WebAssembly
* https://opensource.com/article/19/10/choose-rust-programming-language?utm_campaign=intrel
* https://msrc-blog.microsoft.com/2019/11/07/using-rust-in-windows/
* https://rustwasm.github.io/docs/wasm-bindgen/introduction.html
* https://blog.ryanlevick.com/posts/wasm-bindgen-interop/
