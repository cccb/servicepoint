# ServicePoint

In [CCCB](https://berlin.ccc.de/), there is a big pixel matrix hanging on the wall. It is called  "Service Point
Display" or "Airport Display".
This crate contains C# bindings for the `servicepoint` library, enabling users to parse, encode and send packets to this display via UDP.

## Examples

```csharp
using ServicePoint;

using var connection = Connection.Open("127.0.0.1:2342");
using var pixels = new Bitmap(Constants.PixelWidth, Constants.PixelHeight);

while (true)
{
    pixels.Fill(true);
    connection.Send(Command.BitmapLinearWin(0, 0, pixels.Clone()));
    Thread.Sleep(5000);

    pixels.Fill(false);
    connection.Send(Command.BitmapLinearWin(0, 0, pixels));
    Thread.Sleep(5000);
}
```

An example including project files is available as part of this crate.

You can also check out the unit tests for usage examples for some things.

## Installation

NuGet packages are not a good way to distribute native projects ([relevant issue](https://github.com/dotnet/sdk/issues/33845)).
Because of that, there is no NuGet package you can use directly.
Including this repository as a submodule and building from source is the recommended way of using the library.

```bash
git submodule add https://github.com/cccb/servicepoint.git
git commit -m "add servicepoint submodule"
```

You can now reference `servicepoint_binding_cs/src/ServicePoint.csproj` in your project.
The rust library will automatically be built.

Please provide more information in the form of an issue if you need the build to copy a different library file for your platform.

## Note on stability

This library is still in early development.
You can absolutely use it, and it works, but expect minor breaking changes with every version bump.

## Documentation

There are multiple suboptimal ways to read the documentation for this.

You can read the [rust docs](https://docs.rs/servicepoint/latest/servicepoint/), as the types and methods in C# should have the same names as those in rust.

You can also read the documentation comments on each method. 
Those are copied from the C API, which means they will include the `this` parameter in the description.
They are markdown formatted and may render in one line in your IDE - this is a known [issue](https://github.com/cccb/servicepoint/issues/17).

### Differences to other supported languages

C# does have some differences, especially regarding safety.
In rust, the compiler will tell you when trying to use an object that has already been dropped or moved.
In the C API, the user promises to keep things like that in mind, 
and will get assertion failures or segmentation faults if you are lucky when doing something wrong.

The C# compiler will not help you to keep track of the lifetime of rust objects, but you also do not have to worry about
unnoticed memory corruption in most cases, as the library knows when you pass objects to methods that consume them and
raises a `NullReferenceException` instead.
When objects are garbage collected on the C# side, the rust object is freed as well.

Currently, lifetime tracking may not work reliably in multithreaded code. You should prevent concurrent write access.

## Everything else

Look at the main project [README](https://github.com/cccb/servicepoint/blob/main/README.md) for further information.
