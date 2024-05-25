use std::env;

use cbindgen::{generate_with_config, Config};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = Config::from_file(crate_dir.clone() + "/cbindgen.toml").unwrap();
    let servicepoint_dir = crate_dir.clone() + "/../servicepoint";

    generate_with_config(servicepoint_dir, config)
      .unwrap()
      .write_to_file("servicepoint.h");
}
