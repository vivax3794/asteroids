use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("wasm") {
        println!("cargo:rustc-link-arg=-lm"); // cant use a LLD args for the wasm linker :P
    }
}
