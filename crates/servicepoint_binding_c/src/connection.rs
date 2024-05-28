//! C functions for interacting with `Connection`s
//!
//! prefix `sp_connection_`

use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

use servicepoint::{Connection, Packet};

/// Creates a new instance of `Connection`.
///
/// returns: NULL if connection fails, or connected instance
///
/// # Panics
///
/// Bad string encoding
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_connection_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_connection_open(
    host: *const c_char,
) -> *mut Connection {
    let host = CStr::from_ptr(host).to_str().expect("Bad encoding");
    let connection = match Connection::open(host) {
        Err(_) => return null_mut(),
        Ok(value) => value,
    };

    Box::into_raw(Box::new(connection))
}

/// Sends a `Packet` to the display using the `Connection`.
/// The passed `Packet` gets consumed.
///
/// returns: true in case of success
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `connection` points to a valid instance of `Connection`
/// - `packet` points to a valid instance of `Packet`
/// - `packet` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send(
    connection: *const Connection,
    packet: *mut Packet,
) -> bool {
    let packet = Box::from_raw(packet);
    (*connection).send(*packet).is_ok()
}

/// Closes and deallocates a `Connection`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Connection`
/// - `this` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_connection_dealloc(ptr: *mut Connection) {
    _ = Box::from_raw(ptr);
}
