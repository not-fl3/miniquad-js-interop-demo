# Rust/Javascript interop demo with miniquad

Build instructions:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/miniquad-js-interop-demo.wasm js/demo.wasm
cd js
cargo install basic-http-server # optional, any http server will works
basic-http-server . # or any other http server to server static files with wasm MIME
```
