#![recursion_limit = "512"]

use std::fs;

use html::root::Html;

fn main() {
    const SCRIPT: &str = "import init, {} from './web-panic.js';\
                          init('./web-panic_bg.wasm');";

    let html = Html::builder()
        .head(|h| h.script(|s| s.type_("module").push(SCRIPT)))
        .body(|b| b)
        .build();

    fs::write("./html/index.html", html.to_string()).unwrap();
}
