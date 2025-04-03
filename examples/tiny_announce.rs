//! An example for how to send text to the display - but optimized for minimal binary size.
//!
//! The bulk of optimizations are compiler options, though there are some code changes that together
//! make a huge difference.
//!
//! To build this example inside this repository for the smallest possible size, you can run:
//! ```sh
//! RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" \
//!     rustup run nightly cargo build \
//!     --example=tiny_announce \
//!     --profile=size-optimized \
//!     --no-default-features --features=protocol_udp \
//!     -Zbuild-std="core,std,alloc,proc_macro,panic_abort" \
//!     -Zbuild-std-features="panic_immediate_abort"
//!```
//!
//! Which resulted in a binary size of 8,1k in `rustc 1.88.0-nightly (5e17a2a91 2025-04-05)`.
//!
//! ### Cargo invocation explained
//!
//! - `RUSTFLAGS`: used to pass compiler options to all invocations of `rustc`, that cannot be passed through cargo normally.
//!     - `-Zlocation-detail=none`: (unstable) Removes all info from `caller_location` feature, mostly affecting panic messages
//!     - `-Zfmt-debug=none`: (unstable) Derived `Debug` does nothing, mostly affecting logging panic messages
//! - `rustup run nightly cargo build`: run cargo from the nightly toolchain, similar to `cargo +nightly ...`, but without requiring a wrapper in PATH
//! - `--example=tiny_announce`: build this example only (required because `--target` is specified)
//! - `--profile=size-optimized`: use the profile with the stable options available through `Cargo.toml` as explained below
//! - `--no-default-features --features=protocol_udp`: remove parts of the library (and dependencies) that are not needed
//! - `-Zbuild-std`: (unstable) Do not use the precompiled stdlib to also apply the compiler options there. Without this, the whole stdlib built for execution speed would be included in the binary. This decreases binary size by 285KB.
//! - `-Zbuild-std-features="panic_immediate_abort"`: This is required for the `panic='abort'` option in `Cargo.toml`, when `build-std` is also used.
//!
//! ### Profile `size-optimized`
//!
//! Some options can also be used in stable rust.
//! Those can be specified as part of the Cargo.toml as a profile.
//!
//! ```toml
//! [profile.size-optimized]
//! inherits = "release" # default to release mode for other options
//! opt-level = 'z'      # Optimize for size, disable loop vectorization
//! lto = true           # Enable link-time optimization
//! codegen-units = 1    # Reduce number of codegen units to increase optimizations
//! panic = 'abort'      # Abort on panic
//! strip = true         # Strip symbols from binary
//! ```
//!
//! As this is stable, you can easily copy this to your project, but you will probably not see a
//! giant difference without `build-std`.

#![no_main]

use servicepoint::{
    CharGrid, CharGridCommand, ClearCommand, Connection, UdpConnection,
};
use std::net::SocketAddr;

/// This is the entry point of the example.
/// `#![no_main]` is used to remove the default rust main (6KB)
/// Because we use `#![no_main]`, this is a C-style main function.
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // not parsing the address from str removes 3KB
    let addr = SocketAddr::from(([172, 23, 42, 29], 80));

    let connection = UdpConnection::open(addr).unwrap();
    connection.send(ClearCommand).unwrap();

    // directly initializing to the wanted values (not using `.set()`) saves 0.4KB
    let grid = CharGrid::from_vec(5, vec!['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']).unwrap();

    connection.send(CharGridCommand::from(grid)).unwrap();
    0
}
