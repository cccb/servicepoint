extern crate cbindgen;

use std::env;

use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = env::var("SP2_INCLUDE_DIR")
        .or(env::var("OUT_DIR"))
        .unwrap();

    println!("cargo::warning={out_dir}");

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-env-changed=SP2_INCLUDE_DIR");
    println!("cargo::rerun-if-env-changed=OUT_DIR");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_item_prefix("sp2_")
        .with_language(Language::C)
        .with_cpp_compat(true)
        .with_parse_expand_all_features(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{out_dir}/sp2-bindings.h"));
}