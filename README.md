# web-panic
Panic handling in the browser (wasm)

```bash
cargo install https
cargo run --example index
cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir html --target web --no-typescript --omit-default-module-path target/wasm32-unknown-unknown/debug/web-panic.wasm
http html/
```
