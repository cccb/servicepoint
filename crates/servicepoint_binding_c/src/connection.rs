use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

pub use servicepoint::Connection;
use servicepoint::Packet;

/// Creates a new instance of Connection.
/// The returned instance has to be deallocated with `connection_dealloc`.
///
/// returns: NULL if connection fails or connected instance
///
/// Panics: bad string encoding
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

/// Sends the command instance. The instance is consumed / destroyed and cannot be used after this call.
#[no_mangle]
pub unsafe extern "C" fn sp_connection_send(
    connection: *const Connection,
    command_ptr: *mut Packet,
) -> bool {
    let packet = Box::from_raw(command_ptr);
    (*connection).send(*packet).is_ok()
}

/// Closes and deallocates a connection instance
#[no_mangle]
pub unsafe extern "C" fn sp_connection_dealloc(ptr: *mut Connection) {
    _ = Box::from_raw(ptr);
}
