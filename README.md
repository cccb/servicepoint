# servicepoint2

[![crates.io](https://img.shields.io/crates/v/servicepoint2.svg)](https://crates.io/crates/servicepoint2)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/servicepoint2)](https://crates.io/crates/servicepoint2)
[![docs.rs](https://img.shields.io/docsrs/servicepoint2)](https://docs.rs/servicepoint2/latest/servicepoint2/)
[![GPLv3 licensed](https://img.shields.io/crates/l/servicepoint2)](./LICENSE)

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This repository contains a library for parsing, encoding and sending packets to this display via UDP.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.
Please specify the full version including patch in your Cargo.toml until 1.0 is released.

Expect bugs and/or missing features in the language bindings for now. If you need something specific, open an issue or a pull request.

## Rust

This is where the library works the best.
Any API usage accepted by the compiler in a safe context is either safe or buggy (issues welcome)

```bash
cargo add servicepoint2
```

```rust
fn main() {
    // establish connection
    let connection = servicepoint2::Connection::open("172.23.42.29:2342")
        .expect("connection failed");

    // clear screen content
    connection.send(servicepoint2::Command::Clear)
        .expect("send failed");
}
```

More examples are available in the repository folder and in the [Projects using the library]() section

## C / C++

The lowest common denominator. Things to keep in mind:

- This is a chainsaw. You will cut your leg.
- function names are: `sp2_` \<struct_name\> \<rust name\>.
- Use the rust documentation.
- Instances get consumed in the same way they do when writing rust / C# code. Do not use an instance after an (implicit!) free.
- Option<T> or Result<T, E> turn into nullable return values - check for NULL!
- There are no specifics for C++ here yet. You might get a nicer header when generating directly for C++, but it should be usable.
- Reading and writing to instances concurrently is not safe. Only reading concurrently is safe.

```c++
#include <stdio.h>
#include "servicepoint2.h"

int main(void) {
    sp2_Connection *connection = sp2_connection_open("localhost:2342");
    if (connection == NULL)
        return 1;

    sp2_PixelGrid *pixels = sp2_pixel_grid_new(sp2_PIXEL_WIDTH, sp2_PIXEL_HEIGHT);
    sp2_pixel_grid_fill(pixels, true);
    
    command = sp2_command_bitmap_linear_win(0, 0, pixels); // pixels get consumed here
    if (command == NULL)
        return 4;
    if (!sp2_connection_send(connection, command)) // command gets consumed here
        return 5;

    // connection does not get consumed and has to be freed manually
    sp2_connection_dealloc(connection); 
    return 0;
}
```

## C# / F#

Uses C bindings internally to provide a similar API to rust. Things to keep in mind:

- You will get a `NullPointerException` when trying to call a method where the native instance has been consumed already (e.g. when `Send`ing a command instance twice). Send a clone instead of the original if you want to keep using it.
- Some lower-level APIs _will_ panic in native code when used improperly.
  Example: manipulating the `Span<byte>` of an object after freeing the instance.
- C# specifics are documented in the library. Use the rust documentation for everything else. Naming and semantics are the same apart from CamelCase instead of kebab_case.
- You will only get rust backtraces in debug builds of the native code.
- F# is not explicitly tested. If there are usability or functionality problems, please open an issue.
- Reading and writing to instances concurrently is not safe. Only reading concurrently is safe.

```csharp
using ServicePoint2;

// using statement calls Dispose() on scope exit, which frees unmanaged instances
using var connection = Connection.Open("127.0.0.1:2342");
using var pixels = PixelGrid.New(Constants.PixelWidth, Constants.PixelHeight);

while (true)
{
    pixels.Fill(true);
    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone()));
    Thread.Sleep(5000);

    pixels.Fill(false);
    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone()));
    Thread.Sleep(5000);
}
```

### Installation

NuGet packages are not a good way to distribute native projects ([relevant issue](https://github.com/dotnet/sdk/issues/33845)).
Because of that, there is no NuGet package you can use directly.
Including this repository as a submodule and building from source is the recommended way of using the library.

```bash
git submodule add https://github.com/kaesaecracker/servicepoint.git
git commit -m "add servicepoint submodule"
```

You can now reference `servicepoint2-bindings-cs/src/ServicePoint2.csproj` in your project.
The rust library will automatically be built.

Please provide more information in the form of an issue if you need the build to copy a different library file for your platform.

### Installation

Copy the header to your project and compile against.

You have the choice of linking statically (recommended) or dynamically.
- The C example shows how to link statically against the `staticlib` variant.
- When linked dynamically, you have to provide the `cdylib` at runtime in the _same_ version, as there are no API/ABI guarantees yet.

## Features

This library has multiple compression libraries as optional dependencies.
If you do not need compression/decompression support you can disable those features.
In the likely case you only need one of them, you can include that one specifically.

```toml
[dependencies.servicepoint2]
git = "https://github.com/kaesaecracker/servicepoint.git"
default-features = false
features = ["compression-bz"]
```

Language bindings will not know which features are available and may fail at runtime.
It is recommended to include all features for builds used outside of rust.

## Projects using the library

- screen simulator (rust): https://github.com/kaesaecracker/pixel-receiver-rs
- tanks game (C#): https://github.com/kaesaecracker/cccb-tanks-cs

To add yourself to the list, open a pull request.

## Where is servicepoint1?

This library is a spiritual mix of a not-yet-working rust library called `servicepoint` and a bunch of working but also unfinished C# code. Because most of the API concept and a bunch of code is taken from the rust library, the result is called `servicepoint2`.

## Contributing

Contributions are accepted in any form (issues, documentation, feature requests, code, review, ...).

All creatures welcome.
