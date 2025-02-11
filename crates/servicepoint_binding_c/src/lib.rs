//! C API wrapper for the [servicepoint](https://docs.rs/servicepoint/latest/servicepoint/) crate.
//!
//! # Examples
//!
//! Make sure to check out [this GitHub repo](https://github.com/arfst23/ServicePoint) as well!
//!
//! ```C
//! #include <stdio.h>
//! #include "servicepoint.h"
//!
//! int main(void) {
//!     SPConnection *connection = sp_connection_open("172.23.42.29:2342");
//!     if (connection == NULL)
//!         return 1;
//!
//!     SPBitmap *pixels = sp_bitmap_new(SP_PIXEL_WIDTH, SP_PIXEL_HEIGHT);
//!     sp_bitmap_fill(pixels, true);
//!
//!     SPCommand *command = sp_command_bitmap_linear_win(0, 0, pixels, Uncompressed);
//!     while (sp_connection_send_command(connection, sp_command_clone(command)));
//!
//!     sp_command_free(command);
//!     sp_connection_free(connection);
//!     return 0;
//! }
//! ```

pub use crate::bitmap::*;
pub use crate::bitvec::*;
pub use crate::brightness_grid::*;
pub use crate::byte_slice::*;
pub use crate::char_grid::*;
pub use crate::command::*;
pub use crate::connection::*;
pub use crate::constants::*;
pub use crate::cp437_grid::*;
pub use crate::packet::*;

mod bitmap;
mod bitvec;
mod brightness_grid;
mod byte_slice;
mod char_grid;
mod command;
mod connection;
mod constants;
mod cp437_grid;
mod packet;
