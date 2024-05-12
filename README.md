# servicepoint

[![crates.io](https://img.shields.io/crates/v/servicepoint2.svg)](https://crates.io/crates/servicepoint2)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint2)](https://crates.io/crates/servicepoint2)
[![docs.rs](https://img.shields.io/docsrs/servicepoint2)](https://docs.rs/servicepoint2/latest/servicepoint2/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint2)](./LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This repository contains a library for parsing, encoding and sending packets to this display via UDP.

This library is still in early development.
You can absolutely use it and it works, but expect minor breaking changes with every version bump.
Please specify the full version including patch in your Cargo.toml until 1.0 is released.

### Installation

```bash
# release version
cargo add servicepoint2

# development version
cargo add --git https://github.com/kaesaecracker/servicepoint.git
```

### Example

```rust
fn main() {
    // establish connection
    let connection = servicepoint2::Connection::open("172.23.42.29:2342")
        .expect("connection failed");

    // clear screen content
    connection.send(servicepoint2::Command::Clear)
        .expect("send failed");
}
```

More are available in the `examples` folder.

### Features

This library has multiple compression libraries as optional dependencies.
If you do not need compression/decompression support you can disable those features.
In the likely case you only need one of them, you can include that one specifically.

```toml
[dependencies.servicepoint2]
git = "https://github.com/kaesaecracker/servicepoint.git"
default-features = false
features = ["compression-bz"]
```

### Projects using the library

- screen emulator: https://github.com/kaesaecracker/pixel-receiver-rs
