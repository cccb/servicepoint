//! C functions for interacting with `Packet`s
//!
//! prefix `sp_packet_`

use std::ptr::null_mut;

use crate::command::SPCommand;

/// The raw packet
pub struct SPPacket(pub(crate) servicepoint::Packet);

/// Turns a `Command` into a `Packet`.
/// The `Command` gets consumed.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `command` points to a valid instance of `Command`
/// - `command` is not used concurrently or after this call
/// - the returned `Packet` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_from_command(
    command: *mut SPCommand,
) -> *mut SPPacket {
    let command = *Box::from_raw(command);
    let packet = SPPacket(command.0.into());
    Box::into_raw(Box::new(packet))
}

/// Tries to load a `Packet` from the passed array with the specified length.
///
/// returns: NULL in case of an error, pointer to the allocated packet otherwise
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory region of at least `length` bytes
/// - `data` is not written to concurrently
/// - the returned `Packet` instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_try_load(
    data: *const u8,
    length: usize,
) -> *mut SPPacket {
    let data = std::slice::from_raw_parts(data, length);
    match servicepoint::Packet::try_from(data) {
        Err(_) => null_mut(),
        Ok(packet) => Box::into_raw(Box::new(SPPacket(packet))),
    }
}

/// Clones a `Packet`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Packet`
/// - `this` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_dealloc`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_clone(
    this: *const SPPacket,
) -> *mut SPPacket {
    Box::into_raw(Box::new(SPPacket((*this).0.clone())))
}

/// Deallocates a `Packet`.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `this` points to a valid `Packet`
/// - `this` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_packet_dealloc(this: *mut SPPacket) {
    _ = Box::from_raw(this)
}
