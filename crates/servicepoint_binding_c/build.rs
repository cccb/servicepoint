use std::{env, fs::copy};

use cbindgen::{generate_with_config, Config};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo::rerun-if-changed={crate_dir}");

    let config =
        Config::from_file(crate_dir.clone() + "/cbindgen.toml").unwrap();

    let output_dir = env::var("OUT_DIR").unwrap();
    let header_file = output_dir.clone() + "/servicepoint.h";

    generate_with_config(crate_dir, config)
        .unwrap()
        .write_to_file(&header_file);
    println!("cargo:include={output_dir}");

    println!("cargo::rerun-if-env-changed=SERVICEPOINT_HEADER_OUT");
    if let Ok(header_out) = env::var("SERVICEPOINT_HEADER_OUT") {
        let header_copy = header_out + "/servicepoint.h";
        println!("cargo:warning=Copying header to {header_copy}");
        copy(header_file, &header_copy).unwrap();
        println!("cargo::rerun-if-changed={header_copy}");
    }
}
