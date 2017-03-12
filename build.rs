extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    bindgen::Builder::default()
        .no_unstable_rust()
        .header("include/glk.h")
        .generate()
        .expect("failed to create bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("glk.rs"))
        .expect("failed to write bindings")
}
