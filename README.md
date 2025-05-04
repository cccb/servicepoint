# servicepoint

[![Release](https://git.berlin.ccc.de/servicepoint/servicepoint/badges/release.svg)](https://git.berlin.ccc.de/servicepoint/servicepoint/releases)
[![crates.io](https://img.shields.io/crates/v/servicepoint.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint)](./LICENSE)
[![CI](https://git.berlin.ccc.de/servicepoint/servicepoint/badges/workflows/rust.yml/badge.svg)](https://git.berlin.ccc.de/servicepoint/servicepoint)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".

This crate contains a library for parsing, encoding and sending packets to this display via UDP.
The library itself is written in Rust, but can be used from multiple languages
via [language bindings](#supported-language-bindings).

## Examples

```rust no_run
use std::net::UdpSocket;
// everything you need is in the top-level
use servicepoint::{ClearCommand, UdpSocketExt};

fn main() {
  // this should be the IP of the real display @CCCB
  let destination = "172.23.42.29:2342";

  // establish connection
  let connection = UdpSocket::bind_connect(destination).expect("connection failed");

  // clear screen content using the UdpSocketExt
  connection.send_command(ClearCommand).expect("send failed");
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
servicepoint = "0.14.1"
```

## Note on stability

This library can be used for creative project or just to play around with the display.
A decent coverage by unit tests prevents major problems and I also test this with my own projects, which mostly use
up-to-date versions.

That being said, the API is still being worked on.
Expect breaking changes with every minor version bump.
There should be no breaking changes in patch releases, but there may also be features hiding in those.

All of this means for you: please specify the full version including patch in your Cargo.toml until 1.0 is released.

Release notes are published [here](https://git.berlin.ccc.de/servicepoint/servicepoint/releases), please check them before updating.

Currently, this crate requires Rust [v1.70](https://releases.rs/docs/1.70.0/) from June 2023.

## Features

This library has multiple optional dependencies.
You can choose to (not) include them by toggling the related features.

| Name              | Default | Description                                  | Dependencies                                    |
|-------------------|---------|----------------------------------------------|-------------------------------------------------|
| cp437             | true    | Conversion to and from CP-437                | [once_cell](https://crates.io/crates/once_cell) |
| compression_lzma  | true    | Enable additional compression algorithm      | [rust-lzma](https://crates.io/crates/rust-lzma) |
| compression_zlib  | false   | Enable additional compression algorithm      | [flate2](https://crates.io/crates/flate2)       |
| compression_bzip2 | false   | Enable additional compression algorithm      | [bzip2](https://crates.io/crates/bzip2)         |
| compression_zstd  | false   | Enable additional compression algorithm      | [zstd](https://crates.io/crates/zstd)           |
| rand              | false   | `impl Distribution<Brightness> for Standard` | [rand](https://crates.io/crates/rand)           |

Es an example, if you only want zlib compression:

```toml
[dependencies]
servicepoint = { version = "0.14.1", default-features = false, features = ["compression_zlib"] }
```

If you are looking at features to minimize binary size: take a look at the `tiny_announce`-example!

## Supported language bindings

| Language  | Support level | Repo                                                                                                                                             |
|-----------|---------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| .NET (C#) | Full          | [servicepoint-binding-csharp](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-csharp) contains bindings and a `.csproj` to reference |
| C         | Full          | [servicepoint-binding-c](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-c) contains a header and a library to link against          |
| Ruby      | Working       | [servicepoint-binding-ruby](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-ruby) contains bindings                                  |
| Python    | Unsupported   | bindings can be generated from [servicepoint-binding-uniffi](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-uniffi), tested once    |
| Go        | Unsupported   | bindings can be generated from [servicepoint-binding-uniffi](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-uniffi)                 |
| Kotlin    | Unsupported   | bindings can be generated from [servicepoint-binding-uniffi](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-uniffi)                 |
| Swift     | Unsupported   | bindings can be generated from [servicepoint-binding-uniffi](https://git.berlin.ccc.de/servicepoint/servicepoint-binding-uniffi)                 |

## Projects using the library

- [servicepoint-simulator](https://git.berlin.ccc.de/servicepoint/servicepoint-simulator): a screen simulator written in rust 
- [servicepoint-tanks](https://git.berlin.ccc.de/vinzenz/servicepoint-tanks): a multiplayer game written in C# with a second screen in the browser written in React/Typescript 
- [servicepoint-life](https://git.berlin.ccc.de/vinzenz/servicepoint-life): a cellular automata slideshow written in rust 
- [servicepoint-cli](https://git.berlin.ccc.de/servicepoint/servicepoint-cli): a CLI that can:
    - share (stream) your screen
    - send image files with dithering
    - clear the display
    - ...

To add yourself to the list, open a pull request.

You can also check out [awesome-servicepoint](https://github.com/stars/kaesaecracker/lists/awesome-servicepoint) for a
bigger collection of projects, including some not related to this library.

If you have access, there is even more software linked in [the wiki](https://wiki.berlin.ccc.de/LED-Riesendisplay).

Some more related projects:

- [cccb-servicepoint-browser](https://github.com/SamuelScheit/cccb-servicepoint-browser): a partial typescript implementation inspired by this library and browser stream
- [arfst23/ServicePoint](https://github.com/arfst23/ServicePoint): a bunch of projects in C that [used to](https://zerforschen.plus/posts/tiny-binaries-rust/) use the C bindings
    - a CLI tool to display image files on the display or use the display as a TTY
    - a BSD games robots clone
    - a split-flap-display simulator
    - animations that play on the display

## Contributing

You are welcome to contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

## History

### Move to Forgejo

This project moved
to [git.berlin.ccc.de/servicepoint/servicepoint](https://git.berlin.ccc.de/servicepoint/servicepoint).
The [GitHub repository](https://github.com/cccb/servicepoint) remains available as a mirror.


### What happened to servicepoint2?

`servicepoint2` was a fork of `servicepoint`. Since `servicepoint2` has been merged into `servicepoint`, `servicepoint2` did not get any updates.
