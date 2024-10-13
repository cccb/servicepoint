//! C functions for interacting with [SPPacket]s
//!
//! prefix `sp_packet_`

use std::ptr::null_mut;

use crate::SPCommand;

/// The raw packet
pub struct SPPacket(pub(crate) servicepoint::packet::Packet);

/// Turns a [SPCommand] into a [SPPacket].
/// The [SPCommand] gets consumed.
///
/// Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - [SPCommand] points to a valid instance of [SPCommand]
/// - [SPCommand] is not used concurrently or after this call
/// - the returned [SPPacket] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_from_command(
    command: *mut SPCommand,
) -> *mut SPPacket {
    let command = *Box::from_raw(command);
    let packet = SPPacket(command.0.into());
    Box::into_raw(Box::new(packet))
}

/// Tries to load a [SPPacket] from the passed array with the specified length.
///
/// returns: NULL in case of an error, pointer to the allocated packet otherwise
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `data` points to a valid memory region of at least `length` bytes
/// - `data` is not written to concurrently
/// - the returned [SPPacket] instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_try_load(
    data: *const u8,
    length: usize,
) -> *mut SPPacket {
    let data = std::slice::from_raw_parts(data, length);
    match servicepoint::packet::Packet::try_from(data) {
        Err(_) => null_mut(),
        Ok(packet) => Box::into_raw(Box::new(SPPacket(packet))),
    }
}

/// Clones a [SPPacket].
///
/// Will never return NULL.
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `packet` points to a valid [SPPacket]
/// - `packet` is not written to concurrently
/// - the returned instance is freed in some way, either by using a consuming function or
///   by explicitly calling `sp_packet_free`.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_clone(
    packet: *const SPPacket,
) -> *mut SPPacket {
    Box::into_raw(Box::new(SPPacket((*packet).0.clone())))
}

/// Deallocates a [SPPacket].
///
/// # Safety
///
/// The caller has to make sure that:
///
/// - `packet` points to a valid [SPPacket]
/// - `packet` is not used concurrently or after this call
#[no_mangle]
pub unsafe extern "C" fn sp_packet_free(packet: *mut SPPacket) {
    _ = Box::from_raw(packet)
}
