//! C API wrapper for the `servicepoint` crate.

pub use crate::c_slice::SPByteSlice;

pub mod bit_vec;

pub mod brightness_grid;

pub mod command;

pub mod connection;

pub mod packet;

pub mod pixel_grid;

pub mod c_slice;

pub mod cp437_grid;

pub mod constants;

/// Type alias for documenting the meaning of the variable in enum values
pub type SPOffset = usize;
