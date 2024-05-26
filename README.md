# servicepoint

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This repository contains a library for parsing, encoding and sending packets to this display via UDP in multiple
programming languages.

Take a look at the contained crates for language specific information:

| Language | Readme                                                              |
|----------|---------------------------------------------------------------------|
| Rust     | [servicepoint](crates/servicepoint/README.md)                       |
| C / C++  | [servicepoint_binding_c](crates/servicepoint_binding_c/README.md)   |
| C# / F#  | [servicepoint_binding_cs](crates/servicepoint_binding_cs/README.md) | 

## Projects using the library

- screen simulator (rust): https://github.com/kaesaecracker/servicepoint-simulator
- tanks game (C#): https://github.com/kaesaecracker/cccb-tanks-cs
- cellular automata slideshow (rust): https://github.com/kaesaecracker/servicepoint-life

To add yourself to the list, open a pull request.

## Where is servicepoint1?

This library is a spiritual mix of a not-yet-working rust library called `servicepoint` and a bunch of working but also
unfinished C# code. Because most of the API concept and a bunch of code is taken from the rust library, the result is
called `servicepoint`.

## Contributing

Contributions are accepted in any form (issues, documentation, feature requests, code, review, ...).

All creatures welcome.
