# servicepoint

[![crates.io](https://img.shields.io/crates/v/servicepoint.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint)](../../LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This crate contains a library for parsing, encoding and sending packets to this display via UDP.

## Examples

```rust
fn main() {
    // establish connection
    let connection = servicepoint::Connection::open("172.23.42.29:2342")
        .expect("connection failed");

    // clear screen content
    connection.send(servicepoint::Command::Clear)
        .expect("send failed");
}
```

More examples are available in the crate. 
Execute `cargo run --example` for a list of available examples and `cargo run --example <name>` to run one.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.
Please specify the full version including patch in your Cargo.toml until 1.0 is released.

## Installation

```bash
cargo add servicepoint
```

## Features

This library has multiple compression libraries as optional dependencies.
If you do not need compression/decompression support you can disable those features.
In the likely case you only need one of them, you can include that one specifically.

```toml
[dependencies]
servicepoint = { version = "0.7.0", default-features = false, features = ["compression-bz"] }
```

## Everything else

Look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for further information.
