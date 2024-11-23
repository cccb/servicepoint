# ServicePoint

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".

This crate contains bindings for multiple programming languages, enabling non-rust-developers to use the library.

Also take a look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for more
information.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.

## Notes on differences to rust library

- Performance will not be as good as the rust version:
    - most objects are reference counted.
    - objects with mutating methods will also have a MRSW lock
- You will not get rust backtraces in release builds of the native code
- Panic messages will work (PanicException)

## Supported languages

| Language  | Support level | Notes                                                                                           |
|-----------|---------------|-------------------------------------------------------------------------------------------------|
| .NET (C#) | Full          | see dedicated section                                                                           |
| Ruby      | Working       | LD_LIBRARY_PATH has to be set, see example project                                              |
| Python    | Tested once   | Required project file not included. The shared library will be loaded from the script location. |
| Go        | untested      |                                                                                                 |
| Kotlin    | untested      |                                                                                                 |
| Swift     | untested      |                                                                                                 |

## Installation

Including this repository as a submodule and building from source is the recommended way of using the library.

```bash
git submodule add https://github.com/cccb/servicepoint.git
git commit -m "add servicepoint submodule"
```

Run `generate-bindings.sh` to regenerate all bindings. This will also build `libservicepoint.so` (or equivalent on your
platform).

For languages not fully supported, there will be no project file for the library, just the naked source file(s).
If you successfully use a language, please open an issue or PR to add the missing ones.

## .NET (C#)

This is the best supported language.

F# is not tested. If there are usability or functionality problems, please open an issue.

Currently, the project file is hard-coded for Linux and will need tweaks for other platforms (e.g. `.dylib` instead of `.so`).

You do not have to compile or copy the rust crate manually, as building `ServicePoint.csproj` also builds it.

### Example

```csharp
using System.Threading;
using ServicePoint;

var connection = new Connection("127.0.0.1:2342");
connection.Send(Command.Clear());

connection.Send(Command.Brightness(5));

var pixels = Bitmap.NewMaxSized();
for (ulong offset = 0; offset < ulong.MaxValue; offset++)
{
    pixels.Fill(false);

    for (ulong y = 0; y < pixels.Height(); y++)
        pixels.Set((y + offset) % pixels.Width(), y, true);

    connection.Send(Command.BitmapLinearWin(0, 0, pixels));
    Thread.Sleep(14);
}
```

A full example including project files is available as part of this crate.

### Why is there no NuGet-Package?

NuGet packages are not a good way to distribute native
binaries ([relevant issue](https://github.com/dotnet/sdk/issues/33845)).
Because of that, there is no NuGet package you can use directly.
