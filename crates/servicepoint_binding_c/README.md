# servicepoint_binding_c

[![crates.io](https://img.shields.io/crates/v/servicepoint_binding_c.svg)](https://crates.io/crates/servicepoint)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint_binding_c)](https://crates.io/crates/servicepoint)
[![docs.rs](https://img.shields.io/docsrs/servicepoint_binding_c)](https://docs.rs/servicepoint/latest/servicepoint/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint_binding_c)](../../LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. 
It is called  "Service Point Display" or "Airport Display".

This crate contains C bindings for the `servicepoint` library, enabling users to parse, encode and send packets to this display via UDP.

## Examples

```c++
#include <stdio.h>
#include "servicepoint.h"

int main(void) {
    SPConnection *connection = sp_connection_open("172.23.42.29:2342");
    if (connection == NULL)
        return 1;

    SPPixelGrid *pixels = sp_pixel_grid_new(SP_PIXEL_WIDTH, SP_PIXEL_HEIGHT);
    sp_pixel_grid_fill(pixels, true);

    SPCommand *command = sp_command_bitmap_linear_win(0, 0, pixels, Uncompressed);
    while (sp_connection_send_command(connection, sp_command_clone(command)));

    sp_command_free(command);
    sp_connection_free(connection);
    return 0;
}
```

A full example including Makefile is available as part of this crate.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.
Please specify the full version including patch in your Cargo.toml until 1.0 is released.

## Installation

Copy the header to your project and compile against.

You have the choice of linking statically (recommended) or dynamically.
- The C example shows how to link statically against the `staticlib` variant.
- When linked dynamically, you have to provide the `cdylib` at runtime in the _same_ version, as there are no API/ABI guarantees yet.

## Notes on differences to rust library

- function names are: `sp_` \<struct_name\> \<rust name\>.
- Instances get consumed in the same way they do when writing rust code. Do not use an instance after an (implicit!) free.
- Option<T> or Result<T, E> turn into nullable return values - check for NULL!
- There are no specifics for C++ here yet. You might get a nicer header when generating directly for C++, but it should be usable.
- Reading and writing to instances concurrently is not safe. Only reading concurrently is safe.
- documentation is included in the header and available [online](https://docs.rs/servicepoint_binding_c/latest/servicepoint_binding_c/)

## Everything else

Look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for further information.
