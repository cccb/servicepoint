//! C functions for interacting with `SPConnection`s
//!
//! prefix `sp_connection_`

use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

use crate::{SPCommand, SPPacket};

/// A connection to the display.
///
/// # Examples
///
/// ```C
/// CConnection connection = sp_connection_open("172.23.42.29:2342");
/// if (connection != NULL)
///     sp_connection_send(connection, sp_command_clear());
/// ```
pub struct SPConnection(pub(crate) servicepoint::Connection);

/// Creates a new instance of `SPConnection`.
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
///   by explicitly calling `sp_connection_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_connection_open(
    host: *const c_char,
) -> *mut SPConnection {
    let host = CStr::from_ptr(host).to_str().expect("Bad encoding");
    let connection = match servicepoint::Connection::open(host) {
        Err(_) => return null_mut(),
        Ok(value) => value,
    };

    Box::into_raw(Box::new(SPConnection(connection)))
}

/// Sends a `SPPacket` to the display using the `SPConnection`.
/// The passed `SPPacket` gets consumed.
///
/// returns: true in case of success
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `SPConnection` points to a valid instance of `SPConnection`
/// - `SPPacket` points to a valid instance of `SPPacket`
/// - `SPPacket` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send_packet(
    connection: *const SPConnection,
    packet: *mut SPPacket,
) -> bool {
    let packet = Box::from_raw(packet);
    (*connection).0.send((*packet).0).is_ok()
}

/// Sends a `SPCommand` to the display using the `SPConnection`.
/// The passed `SPCommand` gets consumed.
///
/// returns: true in case of success
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `connection` points to a valid instance of `SPConnection`
/// - `command` points to a valid instance of `SPPacket`
/// - `command` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send_command(
    connection: *const SPConnection,
    command: *mut SPCommand,
) -> bool {
    let command = (*Box::from_raw(command)).0;
    (*connection).0.send(command).is_ok()
}

/// Closes and deallocates a `SPConnection`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `SPConnection`
/// - `this` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_connection_free(ptr: *mut SPConnection) {
    _ = Box::from_raw(ptr);
}
