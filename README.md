# Web Panic Hook
Panic handling in the browser (wasm)

```bash
cargo install wasm-bindgen-cli
cargo install https
cargo run --example index
cargo build --example example --target wasm32-unknown-unknown
wasm-bindgen --out-dir html --target web --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/debug/examples/example.wasm
http html/
```
