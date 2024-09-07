# servicepoint

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This repository contains a library for parsing, encoding and sending packets to this display via UDP in multiple
programming languages.

Take a look at the contained crates for language specific information:

| Language  | Readme                                                              |
|-----------|---------------------------------------------------------------------|
| Rust      | [servicepoint](crates/servicepoint/README.md)                       |
| C / C++   | [servicepoint_binding_c](crates/servicepoint_binding_c/README.md)   |
| .NET (C#) | [servicepoint_binding_cs](crates/servicepoint_binding_cs/README.md) | 

## Projects using the library

- screen simulator (rust): [servicepoint-simulator](https://github.com/kaesaecracker/servicepoint-simulator)
- A bunch of projects (C): [arfst23/ServicePoint](https://github.com/arfst23/ServicePoint), including
  - a CLI tool to display image files on the display or use the display as a TTY
  - a BSD games robots clone
  - a split-flap-display simulator
  - animations that play on the display
- tanks game (C#): [servicepoint-tanks](https://github.com/kaesaecracker/cccb-tanks-cs)
- cellular automata slideshow (rust): [servicepoint-life](https://github.com/kaesaecracker/servicepoint-life)

To add yourself to the list, open a pull request.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## What happened to servicepoint2?

After `servicepoint2` has been merged into `servicepoint`, `servicepoint2` will not continue to get any updates.
