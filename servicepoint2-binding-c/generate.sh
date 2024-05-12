#!/usr/bin/env sh

# if the script crashes here, run `cargo install cbindgen`
cbindgen --config cbindgen.toml --clean --output servicepoint2.h ../servicepoint2
