use std::ptr::null_mut;

use servicepoint::Command;
pub use servicepoint::Packet;

/// Turns a `Command` into a `Packet`. The command gets deallocated in the process.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_from_command(
    command: *mut Command,
) -> *mut Packet {
    let command = *Box::from_raw(command);
    let packet = command.into();
    Box::into_raw(Box::new(packet))
}

/// Tries to load a `Packet` from the passed array with the specified length.
///
/// returns: NULL in case of an error, pointer to the allocated packet otherwise
#[no_mangle]
pub unsafe extern "C" fn sp_packet_try_load(
    data: *const u8,
    length: usize,
) -> *mut Packet {
    let data = std::slice::from_raw_parts(data, length);
    match Packet::try_from(data) {
        Err(_) => null_mut(),
        Ok(packet) => Box::into_raw(Box::new(packet)),
    }
}

/// Deallocates a `Packet`.
///
/// Note: do not call this if the instance has been consumed in another way, e.g. by sending it.
#[no_mangle]
pub unsafe extern "C" fn sp_packet_dealloc(this: *mut Packet) {
    _ = Box::from_raw(this)
}
