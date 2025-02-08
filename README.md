# servicepoint

[![crates.io](https://img.shields.io/crates/v/servicepoint.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint)](./LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This repository contains a library for parsing, encoding and sending packets to this display via UDP in multiple
programming languages.

This repository will move to [git.berlin.ccc.de/servicepoint/servicepoint](https://git.berlin.ccc.de/servicepoint/servicepoint) soon.

Take a look at the contained crates for language specific information:

| Crate                       | Languages                         | Readme                                                                      |
|-----------------------------|-----------------------------------|-----------------------------------------------------------------------------|
| servicepoint                | Rust                              | [servicepoint](crates/servicepoint/README.md)                               |
| servicepoint_binding_c      | C / C++                           | [servicepoint_binding_c](crates/servicepoint_binding_c/README.md)           |
| servicepoint_binding_uniffi | C# / Python / Go / Kotlin / Swift | [servicepoint_binding_uniffi](crates/servicepoint_binding_uniffi/README.md) |

## Projects using the library

- screen simulator (rust): [servicepoint-simulator](https://github.com/kaesaecracker/servicepoint-simulator)
- A bunch of projects (C): [arfst23/ServicePoint](https://github.com/arfst23/ServicePoint), including
    - a CLI tool to display image files on the display or use the display as a TTY
    - a BSD games robots clone
    - a split-flap-display simulator
    - animations that play on the display
- tanks game (C#): [servicepoint-tanks](https://github.com/kaesaecracker/cccb-tanks-cs)
- cellular automata slideshow (rust): [servicepoint-life](https://github.com/kaesaecracker/servicepoint-life)
- partial typescript implementation inspired by this library and browser stream: [cccb-servicepoint-browser](https://github.com/SamuelScheit/cccb-servicepoint-browser)
- a CLI: [servicepoint-cli](https://git.berlin.ccc.de/servicepoint/servicepoint-cli)

To add yourself to the list, open a pull request.

You can also check out [awesome-servicepoint](https://github.com/stars/kaesaecracker/lists/awesome-servicepoint) for a bigger collection of projects, including some not related to this library.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## What happened to servicepoint2?

After `servicepoint2` has been merged into `servicepoint`, `servicepoint2` will not continue to get any updates.
