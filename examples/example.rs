use std::panic;

fn main() {
    panic::set_hook(Box::new(web_panic_hook::hook));

    panic!("Hello, world!");
}
