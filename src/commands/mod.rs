mod bitmap;
mod bitmap_legacy;
mod bitvec;
mod brightness;
mod brightness_grid;
mod char_grid;
mod clear;
mod cp437_grid;
mod fade_out;
mod hard_reset;
mod typed;

use crate::command_code::CommandCode;
use crate::*;
use std::fmt::Debug;

pub use bitmap::*;
pub use bitmap_legacy::*;
pub use bitvec::*;
pub use brightness::*;
pub use brightness_grid::*;
pub use char_grid::*;
pub use clear::*;
pub use cp437_grid::*;
pub use fade_out::*;
pub use hard_reset::*;
pub use typed::*;

/// This trait represents a command that can be sent to the display.
///
/// To send a [Command], use a [connection][crate::Connection].
///
/// # Available commands
///
/// To send text, take a look at [Cp437GridCommand].
///
/// To draw pixels, the easiest command to use is [BitmapCommand].
///
/// The other BitmapLinear-Commands operate on a region of pixel memory directly.
/// [BitVecCommand] overwrites a region.
/// [BitmapLinearOr], [BitmapLinearAnd] and [BitmapLinearXor] apply logical operations per pixel.
///
/// Out of bounds operations may be truncated or ignored by the display.
///
/// # Compression
///
/// Some commands can contain compressed payloads.
/// To get started, use [CompressionCode::default].
///
/// If you want to archive the best performance (e.g. latency),
/// you can try the different compression algorithms for your hardware and use case.
///
/// In memory, the payload is not compressed in the [Command].
/// Payload (de-)compression happens when converting the [Command] into a [Packet] or vice versa.
///
/// # Examples
///
/// ```rust
/// use servicepoint::*;
///
/// // create command
/// let command = BrightnessCommand{ brightness: Brightness::MAX };
///
/// // turn command into Packet
/// let packet: Packet = command.clone().into();
///
/// // read command from packet
/// let round_tripped = TypedCommand::try_from(packet).unwrap();
///
/// // round tripping produces exact copy
/// assert_eq!(round_tripped, TypedCommand::from(command.clone()));
///
/// // send command
/// # let connection = FakeConnection;
/// connection.send(command).unwrap();
/// ```
pub trait Command: Debug + Clone + PartialEq + Into<Packet> {}

impl<T: Debug + Clone + PartialEq + Into<Packet>> Command for T {}

fn check_command_code_only(
    packet: Packet,
    code: CommandCode,
) -> Option<TryFromPacketError> {
    let Packet {
        header:
            Header {
                command_code: _,
                a,
                b,
                c,
                d,
            },
        payload,
    } = packet;
    if packet.header.command_code != u16::from(code) {
        Some(TryFromPacketError::InvalidCommand(
            packet.header.command_code,
        ))
    } else if !payload.is_empty() {
        Some(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
    } else if a != 0 || b != 0 || c != 0 || d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::command_code::CommandCode;
    use crate::commands::{BinaryOperation, TryFromPacketError};
    use crate::*;

    pub(crate) fn round_trip(original: TypedCommand) {
        let packet: Packet = original.clone().into();
        let copy: TypedCommand = match TypedCommand::try_from(packet) {
            Ok(command) => command,
            Err(err) => panic!("could not reload {original:?}: {err:?}"),
        };
        assert_eq!(copy, original);
    }
}
