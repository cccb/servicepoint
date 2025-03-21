//! Raw packet manipulation.
//!
//! Should probably only be used directly to use features not exposed by the library.
//!
//! # Examples
//!
//! Converting a packet to a command and back:
//!
//! ```rust
//! use servicepoint::{Command, Packet, TypedCommand};
//! # let command = servicepoint::ClearCommand;
//! let packet: Packet = command.into();
//! let command = TypedCommand::try_from(packet).expect("could not read command from packet");
//! ```
//!
//! Converting a packet to bytes and back:
//!
//! ```rust
//! use servicepoint::{Command, Packet};
//! # let command = servicepoint::ClearCommand;
//! # let packet: Packet = command.into();
//! let bytes: Vec<u8> = packet.into();
//! let packet = Packet::try_from(bytes).expect("could not read packet from bytes");
//! ```

use crate::{command_code::CommandCode, Grid, Origin, Tiles};
use std::{mem::size_of, num::TryFromIntError};

/// A raw header.
///
/// The header specifies the kind of command, the size of the payload and where to display the
/// payload, where applicable.
///
/// Because the meaning of most fields depend on the command, there are no speaking names for them.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Header {
    /// The first two bytes specify which command this packet represents.
    pub command_code: u16,
    /// First command-specific value
    pub a: u16,
    /// Second command-specific value
    pub b: u16,
    /// Third command-specific value
    pub c: u16,
    /// Fourth command-specific value
    pub d: u16,
}

/// The raw payload.
///
/// Should probably only be used directly to use features not exposed by the library.
pub type Payload = Vec<u8>;

/// The raw packet.
///
/// Contents should probably only be used directly to use features not exposed by the library.
///
/// You may want to use [crate::Command] or [crate::TypedCommand] instead.
#[derive(Clone, Debug, PartialEq)]
pub struct Packet {
    /// Meta-information for the packed command
    pub header: Header,
    /// The data for the packed command
    pub payload: Payload,
}

impl From<Packet> for Vec<u8> {
    /// Turn the packet into raw bytes ready to send
    fn from(value: Packet) -> Self {
        let Packet {
            header:
                Header {
                    command_code: mode,
                    a,
                    b,
                    c,
                    d,
                },
            payload,
        } = value;

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

impl TryFrom<&[u8]> for Packet {
    type Error = ();

    /// Tries to interpret the bytes as a [Packet].
    ///
    /// returns: `Error` if slice is not long enough to be a [Packet]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < size_of::<Header>() {
            return Err(());
        }

        let header = {
            let command_code = Self::u16_from_be_slice(&value[0..=1]);
            let a = Self::u16_from_be_slice(&value[2..=3]);
            let b = Self::u16_from_be_slice(&value[4..=5]);
            let c = Self::u16_from_be_slice(&value[6..=7]);
            let d = Self::u16_from_be_slice(&value[8..=9]);
            Header {
                command_code,
                a,
                b,
                c,
                d,
            }
        };
        let payload = value[10..].to_vec();

        Ok(Packet { header, payload })
    }
}

impl TryFrom<Vec<u8>> for Packet {
    type Error = ();

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}

impl Packet {
    fn u16_from_be_slice(slice: &[u8]) -> u16 {
        let mut bytes = [0u8; 2];
        bytes[0] = slice[0];
        bytes[1] = slice[1];
        u16::from_be_bytes(bytes)
    }

    pub(crate) fn origin_grid_to_packet<T>(
        origin: Origin<Tiles>,
        grid: impl Grid<T> + Into<Payload>,
        command_code: CommandCode,
    ) -> Result<Packet, TryFromIntError> {
        Ok(Packet {
            header: Header {
                command_code: command_code.into(),
                a: origin.x.try_into()?,
                b: origin.y.try_into()?,
                c: grid.width().try_into()?,
                d: grid.height().try_into()?,
            },
            payload: grid.into(),
        })
    }

    pub(crate) fn command_code_only(c: CommandCode) -> Self {
        Self {
            header: Header {
                command_code: c.into(),
                ..Default::default()
            },
            payload: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let p = Packet {
            header: Header {
                command_code: 0,
                a: 1,
                b: 2,
                c: 3,
                d: 4,
            },
            payload: vec![42u8; 23],
        };
        let data: Vec<u8> = p.into();
        let p = Packet::try_from(data).unwrap();
        assert_eq!(
            p,
            Packet {
                header: Header {
                    command_code: 0,
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4
                },
                payload: vec![42u8; 23]
            }
        );
    }

    #[test]
    fn too_small() {
        let data = vec![0u8; 4];
        assert_eq!(Packet::try_from(data.as_slice()), Err(()))
    }
}
