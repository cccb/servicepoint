# About the display

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
