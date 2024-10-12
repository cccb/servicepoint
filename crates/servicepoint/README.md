# servicepoint

[![crates.io](https://img.shields.io/crates/v/servicepoint.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint)](../../LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This crate contains a library for parsing, encoding and sending packets to this display via UDP.

## Installation

```bash
cargo add servicepoint
```
or
```toml
[dependencies]
servicepoint = "0.9.0"
```

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

## Features

This library has multiple optional dependencies.
You can choose to (not) include them by toggling the related features.

| Name               | Default | Description                                |
|--------------------|---------|--------------------------------------------|
| compression_zlib   | false   | Enable additional compression algo         |
| compression_bzip2  | false   | Enable additional compression algo         |
| compression_lzma   | true    | Enable additional compression algo         |
| compression_zstd   | false   | Enable additional compression algo         |
| protocol_udp       | true    | Connection::Udp                            |
| protocol_websocket | false   | Connection::WebSocket                      |
| rand               | false   | impl Distribution<Brightness> for Standard |
| cp437              | true    | Conversion to and from CP-437              |

## Everything else

Look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for further information.
