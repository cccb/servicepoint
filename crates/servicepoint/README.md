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
servicepoint = "0.13.0"
```

## Examples

```rust no_run
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

This library can be used for creative project or just to play around with the display.
A decent coverage by unit tests prevents major problems and I also test this with my own projects, which mostly use up-to-date versions.

That being said, the API is still being worked on.
Expect minor breaking changes with every version bump.
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

Look at the main project [README](https://git.berlin.ccc.de/servicepoint/servicepoint/src/branch/main/README.md) for further information.
