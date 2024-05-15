use std::mem::size_of;

/// A raw header. Should probably not be used directly.
#[derive(Debug)]
pub struct Header(pub u16, pub u16, pub u16, pub u16, pub u16);

/// The raw payload. Should probably not be used directly.
pub type Payload = Vec<u8>;

/// The raw packet. Should probably not be used directly.
#[derive(Debug)]
pub struct Packet(pub Header, pub Payload);

impl From<Packet> for Vec<u8> {
    /// Turn the packet into raw bytes ready to send
    fn from(value: Packet) -> Self {
        let Packet(Header(mode, a, b, c, d), payload) = value;

        let mut packet = vec![0u8; 10 + payload.len()];
        packet[0..=1].copy_from_slice(&u16::to_be_bytes(mode));
        packet[2..=3].copy_from_slice(&u16::to_be_bytes(a));
        packet[4..=5].copy_from_slice(&u16::to_be_bytes(b));
        packet[6..=7].copy_from_slice(&u16::to_be_bytes(c));
        packet[8..=9].copy_from_slice(&u16::to_be_bytes(d));

        packet[10..].copy_from_slice(&payload);

        packet
    }
}

fn u16_from_be_slice(slice: &[u8]) -> u16 {
    let mut bytes = [0u8; 2];
    bytes[0] = slice[0];
    bytes[1] = slice[1];
    u16::from_be_bytes(bytes)
}

impl TryFrom<&[u8]> for Packet {
    type Error = ();

    /// Tries to interpret the bytes as a `Packet`.
    ///
    /// returns: `Error` if slice is not long enough to be a `Packet`
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < size_of::<Header>() {
            return Err(());
        }

        let mode = u16_from_be_slice(&value[0..=1]);
        let a = u16_from_be_slice(&value[2..=3]);
        let b = u16_from_be_slice(&value[4..=5]);
        let c = u16_from_be_slice(&value[6..=7]);
        let d = u16_from_be_slice(&value[8..=9]);
        let payload = value[10..].to_vec();

        Ok(Packet(Header(mode, a, b, c, d), payload))
    }
}

#[cfg(feature = "c-api")]
mod c_api {
    use std::ptr::null_mut;

    use crate::{Command, Packet};

    /// Turns a `Command` into a `Packet`. The command gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_packet_from_command(command: *mut Command) -> *mut Packet {
        let command = *Box::from_raw(command);
        let packet = command.into();
        Box::into_raw(Box::new(packet))
    }


    /// Tries to load a `Packet` from the passed array with the specified length.
    ///
    /// returns: NULL in case of an error, pointer to the allocated packet otherwise
    #[no_mangle]
    pub unsafe extern "C" fn sp2_packet_try_load(data: *const u8, length: usize) -> *mut Packet {
        let data = std::slice::from_raw_parts(data, length);
        match Packet::try_from(data) {
            Err(_) => null_mut(),
            Ok(packet) => Box::into_raw(Box::new(packet))
        }
    }

    /// Deallocates a `Packet`.
    ///
    /// Note: do not call this if the instance has been consumed in another way, e.g. by sending it.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_packet_dealloc(this: *mut Packet) {
        _ = Box::from_raw(this)
    }
}