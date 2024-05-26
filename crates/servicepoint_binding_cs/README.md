# ServicePoint

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This crate contains C# bindings for the `servicepoint` library, enabling users to parse, encode and send packets to this display via UDP.

## Examples

```csharp
using ServicePoint;

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

A full example including project files is available as part of this crate.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.

## Installation

NuGet packages are not a good way to distribute native projects ([relevant issue](https://github.com/dotnet/sdk/issues/33845)).
Because of that, there is no NuGet package you can use directly.
Including this repository as a submodule and building from source is the recommended way of using the library.

```bash
git submodule add https://github.com/kaesaecracker/servicepoint.git
git commit -m "add servicepoint submodule"
```

You can now reference `servicepoint-bindings-cs/src/ServicePoint.csproj` in your project.
The rust library will automatically be built.

Please provide more information in the form of an issue if you need the build to copy a different library file for your platform.

## Notes on differences to rust library

Uses C bindings internally to provide a similar API to rust. Things to keep in mind:

- You will get a `NullPointerException` when trying to call a method where the native instance has been consumed already (e.g. when `Send`ing a command instance twice). Send a clone instead of the original if you want to keep using it.
- Some lower-level APIs _will_ panic in native code when used improperly.
  Example: manipulating the `Span<byte>` of an object after freeing the instance.
- C# specifics are documented in the library. Use the rust documentation for everything else. Naming and semantics are the same apart from CamelCase instead of kebab_case.
- You will only get rust backtraces in debug builds of the native code.
- F# is not explicitly tested. If there are usability or functionality problems, please open an issue.
- Reading and writing to instances concurrently is not safe. Only reading concurrently is safe.

## Everything else

Look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for further information.
