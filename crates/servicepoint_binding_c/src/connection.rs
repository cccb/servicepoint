//! C functions for interacting with [SPConnection]s
//!
//! prefix `sp_connection_`

use std::ffi::{c_char, CStr};
use std::ptr::{null_mut, NonNull};

use crate::{SPCommand, SPPacket};

/// A connection to the display.
///
/// # Examples
///
/// ```C
/// CConnection connection = sp_connection_open("172.23.42.29:2342");
/// if (connection != NULL)
///     sp_connection_send_command(connection, sp_command_clear());
/// ```
pub struct SPConnection(pub(crate) servicepoint::Connection);

/// Creates a new instance of [SPConnection] that uses UDP to send.
///
/// returns: NULL if connection fails, or connected instance
///
/// # Panics
///
/// - when `host` is null or an invalid host
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
    assert!(!host.is_null());
    let host = CStr::from_ptr(host).to_str().expect("Bad encoding");
    let connection = match servicepoint::Connection::open(host) {
        Err(_) => return null_mut(),
        Ok(value) => value,
    };

    Box::into_raw(Box::new(SPConnection(connection)))
}

/// Creates a new instance of [SPConnection] for testing that does not actually send anything.
///
/// returns: a new instance. Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_connection_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_connection_fake() -> NonNull<SPConnection> {
    let result = Box::new(SPConnection(servicepoint::Connection::Fake));
    NonNull::from(Box::leak(result))
}

/// Sends a [SPPacket] to the display using the [SPConnection].
///
/// The passed `packet` gets consumed.
///
/// returns: true in case of success
///
/// # Panics
///
/// - when `connection` is NULL
/// - when `packet` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `connection` points to a valid instance of [SPConnection]
/// - `packet` points to a valid instance of [SPPacket]
/// - `packet` is not used concurrently or after this call
///
/// servicepoint_csbindgen_consumes: packet
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send_packet(
    connection: *const SPConnection,
    packet: *mut SPPacket,
) -> bool {
    assert!(!connection.is_null());
    assert!(!packet.is_null());
    let packet = Box::from_raw(packet);
    (*connection).0.send((*packet).0).is_ok()
}

/// Sends a [SPCommand] to the display using the [SPConnection].
///
/// The passed `command` gets consumed.
///
/// returns: true in case of success
///
/// # Panics
///
/// - when `connection` is NULL
/// - when `command` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `connection` points to a valid instance of [SPConnection]
/// - `command` points to a valid instance of [SPPacket]
/// - `command` is not used concurrently or after this call
///
/// servicepoint_csbindgen_consumes: command
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send_command(
    connection: *const SPConnection,
    command: *mut SPCommand,
) -> bool {
    assert!(!connection.is_null());
    assert!(!command.is_null());
    let command = (*Box::from_raw(command)).0;
    (*connection).0.send(command).is_ok()
}

/// Closes and deallocates a [SPConnection].
///
/// # Panics
///
/// - when `connection` is NULL
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `connection` points to a valid [SPConnection]
/// - `connection` is not used concurrently or after this call
///
/// servicepoint_csbindgen_consumes: connection
#[no_mangle]
pub unsafe extern "C" fn sp_connection_free(connection: *mut SPConnection) {
    assert!(!connection.is_null());
    _ = Box::from_raw(connection);
}
