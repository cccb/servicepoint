extern crate cbindgen;

use std::env;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rerun-if-changed=src");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_item_prefix("sp2_")
        .with_language(Language::C)
        .with_cpp_compat(true)
        .with_parse_expand_all_features(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("sp2-bindings.h");
}