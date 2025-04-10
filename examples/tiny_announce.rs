//! An example for how to send text to the display - but optimized for minimal binary size.
//!
//! See [zerforschen.plus/posts/tiny-binaries-rust](https://zerforschen.plus/posts/tiny-binaries-rust/)
//! for details.
//!
//! The bulk of optimizations are compiler options, though there are some code changes that together
//! make a huge difference.
//!
//! To build this example inside this repository for the smallest possible size, you can run:
//! ```sh
//! RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none" \
//!     cargo build \
//!     --example=tiny_announce \
//!     --profile=size-optimized \
//!     --no-default-features --features=protocol_udp \
//!     -Zbuild-std="core,std,alloc,proc_macro,panic_abort" \
//!     -Zbuild-std-features="panic_immediate_abort"
//!```
//!
//! This requires unstable rust.

#![no_main]

use servicepoint::{
    CharGrid, CharGridCommand, ClearCommand, Connection, UdpConnection,
};
use std::net::SocketAddr;

/// This is the entry point of the example.
/// `#![no_main]` is used to remove the default rust main
/// Because we use `#![no_main]`, this is a C-style main function.
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let addr = SocketAddr::from(([172, 23, 42, 29], 2342));

    let connection = UdpConnection::open(addr).unwrap();
    connection.send(ClearCommand).unwrap();

    let grid = CharGrid::from_vec(5, vec!['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd']).unwrap();

    connection.send(CharGridCommand::from(grid)).unwrap();
    0
}
