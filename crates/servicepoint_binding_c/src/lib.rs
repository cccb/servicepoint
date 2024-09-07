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
//!     SPPixelGrid *pixels = sp_pixel_grid_new(SP_PIXEL_WIDTH, SP_PIXEL_HEIGHT);
//!     sp_pixel_grid_fill(pixels, true);
//!
//!     SPCommand *command = sp_command_bitmap_linear_win(0, 0, pixels, Uncompressed);
//!     SPPacket *packet = sp_packet_from_command(command);
//!     while (sp_connection_send(connection, sp_packet_clone(packet)));
//!
//!     sp_packet_dealloc(packet);
//!     sp_connection_dealloc(connection);
//!     return 0;
//! }
//! ```

pub use bit_vec::*;
pub use brightness_grid::*;
pub use byte_slice::*;
pub use command::*;
pub use connection::*;
pub use constants::*;
pub use cp437_grid::*;
pub use packet::*;
pub use pixel_grid::*;

mod bit_vec;

mod brightness_grid;

mod command;

mod connection;

mod packet;

mod pixel_grid;

mod byte_slice;

mod cp437_grid;

mod constants;
