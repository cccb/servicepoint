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

- screen simulator (rust): [servicepoint-simulator](https://github.com/kaesaecracker/servicepoint-simulator)
- tanks game (C#): [servicepoint-tanks](https://github.com/kaesaecracker/cccb-tanks-cs)
- cellular automata slideshow (rust): [servicepoint-life](https://github.com/kaesaecracker/servicepoint-life)

To add yourself to the list, open a pull request.

## About the display

- Resolution: 352x160=56,320 pixels
- Pixels are grouped into 44x20=880 tiles (8x8=64 pixels each)
- Smallest addressable unit: row of pixels inside of a tile (8 pixels = 1 byte)
- The brightness can only be set per tile
- Screen content can be changed using a simple UDP protocol
- Between each row of tiles, there is a gap of around 4 pixels size. This gap changes the aspect ratio of the display.

### Binary format

A UDP package sent to the display has a header size of 10 bytes.
Each header value has a size of two bytes (unsigned 16 bit integer).
Depending on the command, there can be a payload following the header.

The commands are implemented in DisplayCommands.

To change screen contents, these commands are the most relevant:

1. Clear screen
    - command: `0x0002`
    - (rest does not matter)
2. Send CP437 data: render specified text into rectangular region
    - command: `0x0003`
    - top left tile x
    - top left tile y
    - width in tiles
    - height in tiles
    - payload: (width in tiles * height in tiles) bytes
        - 1 byte = 1 character
        - each character is rendered into one tile (mono-spaced)
        - characters are encoded using code page 437
3. Send bitmap window: set pixel states for a rectangular region
    - command: `0x0013`
    - top left tile x
    - top left _pixel_ y
    - width in tiles
    - height in _pixels_
    - payload: (width in tiles * height in pixels) bytes
        - network byte order
        - 1 bit = 1 pixel

There are other commands implemented as well, e.g. for changing the brightness.

## What happened to servicepoint2?

After `servicepoint2` has been merged into `servicepoint`, `servicepoint2` will not continue to get any updates.

## Contributing

Contributions are accepted in any form (issues, documentation, feature requests, code, review, ...).

All creatures welcome.
