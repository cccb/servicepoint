# Contributing

Contributions are accepted in any form (issues, documentation, feature requests, code, review, ...).

All creatures welcome.

## Pull requests

Feel free to create a PR, even if your change is not done yet.

Mark your PR as a draft as long as you do not want it to be merged.

The main branch is supposed to be a working version, including language bindings,
which means sometimes your PR may be merged into a temporary development branch.

Unit tests and documentation are required for the core library.

## Language bindings

Pull requests for your preferred language will be accepted.
If there is no code generator, it should call the C ABI methods provided by `servicepoint_binding_c`.
It should be able to send most of the basic commands in a way the simulator accepts, receiving is
not required for the merge.

It is okay for the feature set of a language binding to lag behind the one of the rust crate.
This also means you do not have to expose a feature to all the language bindings when adding something to the core.

If your change may break other language bindings, please note that in your PR description so someone can check them.
