# servicepoint

[![crates.io](https://img.shields.io/crates/v/servicepoint.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint)](./LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This crate contains a library for parsing, encoding and sending packets to this display via UDP.

This project moved to [git.berlin.ccc.de/servicepoint/servicepoint](https://git.berlin.ccc.de/servicepoint/servicepoint).
The [GitHub repository](https://github.com/cccb/servicepoint) remains available as a mirror.

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

## Installation

```bash
cargo add servicepoint
```
or
```toml
[dependencies]
servicepoint = "0.13.1"
```

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

## Supported language bindings

| Language  | Support level | Repo                                                                                              |
|-----------|---------------|---------------------------------------------------------------------------------------------------|
| .NET (C#) | Full          | [servicepoint-binding-csharp](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-csharp) |
| C         | Full          | [servicepoint-binding-c](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-c)           |
| Ruby      | Working       | [servicepoint-binding-ruby](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-ruby)     |

Other languages should work as well using [servicepoint-binding-uniffi](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-uniffi):
- Python (tested once)
- Go
- Kotlin
- Swift

## Projects using the library

- screen simulator (rust): [servicepoint-simulator](https://git.berlin.ccc.de/servicepoint/servicepoint-simulator)
- A bunch of projects (C): [arfst23/ServicePoint](https://github.com/arfst23/ServicePoint), including
    - a CLI tool to display image files on the display or use the display as a TTY
    - a BSD games robots clone
    - a split-flap-display simulator
    - animations that play on the display
- tanks game (C#): [servicepoint-tanks](https://github.com/kaesaecracker/cccb-tanks-cs)
- cellular automata slideshow (rust): [servicepoint-life](https://github.com/kaesaecracker/servicepoint-life)
- partial typescript implementation inspired by this library and browser
  stream: [cccb-servicepoint-browser](https://github.com/SamuelScheit/cccb-servicepoint-browser)
- a CLI: [servicepoint-cli](https://git.berlin.ccc.de/servicepoint/servicepoint-cli)

To add yourself to the list, open a pull request.

You can also check out [awesome-servicepoint](https://github.com/stars/kaesaecracker/lists/awesome-servicepoint) for a
bigger collection of projects, including some not related to this library.

If you have access, there is even more software linked in [the wiki](https://wiki.berlin.ccc.de/LED-Riesendisplay).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## What happened to servicepoint2?

After `servicepoint2` has been merged into `servicepoint`, `servicepoint2` will not continue to get any updates.
