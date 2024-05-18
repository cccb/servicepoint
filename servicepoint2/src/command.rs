use crate::command_code::CommandCode;
use crate::compression::{into_compressed, into_decompressed};
use crate::{BitVec, ByteGrid, CompressionCode, Grid, Header, Packet, PixelGrid, TILE_SIZE};

/// An origin marks the top left position of a window sent to the display.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Origin(pub u16, pub u16);

/// Size defines the width and height of a window
#[derive(Debug, Clone, Copy)]
pub struct Size(pub u16, pub u16);

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = u16;

/// Type alias for documenting the meaning of the u16 in enum values
pub type Brightness = u8;

/// A command to send to the display.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Set all pixels to the off state
    Clear,
    /// Kills the udp daemon, usually results in a reboot of the display.
    HardReset,
    /// Slowly decrease brightness until off? Untested.
    FadeOut,
    /// Set the brightness of tiles
    CharBrightness(Origin, ByteGrid),
    /// Set the brightness of all tiles
    Brightness(Brightness),
    #[deprecated]
    /// Legacy command code, gets ignored by the real display.
    BitmapLegacy,
    /// Set pixel data starting at the offset.
    /// The contained BitVec is always uncompressed.
    BitmapLinear(Offset, BitVec, CompressionCode),
    /// Set pixel data according to an and-mask starting at the offset.
    /// The contained BitVec is always uncompressed.
    BitmapLinearAnd(Offset, BitVec, CompressionCode),
    /// Set pixel data according to an or-mask starting at the offset.
    /// The contained BitVec is always uncompressed.
    BitmapLinearOr(Offset, BitVec, CompressionCode),
    /// Set pixel data according to an xor-mask starting at the offset.
    /// The contained BitVec is always uncompressed.
    BitmapLinearXor(Offset, BitVec, CompressionCode),
    /// Show text on the screen. Note that the byte data has to be CP437 encoded.
    Cp437Data(Origin, ByteGrid),
    /// Sets a window of pixels to the specified values
    BitmapLinearWin(Origin, PixelGrid, CompressionCode),
}

impl From<Command> for Packet {
    /// Move the `Command` into a `Packet` instance for sending.
    fn from(value: Command) -> Self {
        match value {
            Command::Clear => command_code_only(CommandCode::Clear),
            Command::FadeOut => command_code_only(CommandCode::FadeOut),
            Command::HardReset => command_code_only(CommandCode::HardReset),
            #[allow(deprecated)]
            Command::BitmapLegacy => {
                command_code_only(CommandCode::BitmapLegacy)
            }
            Command::CharBrightness(Origin(x, y), grid) => Packet(
                Header(
                    CommandCode::CharBrightness.into(),
                    x,
                    y,
                    grid.width() as u16,
                    grid.height() as u16,
                ),
                grid.into(),
            ),
            Command::Brightness(brightness) => Packet(
                Header(
                    CommandCode::Brightness.into(),
                    0x00000,
                    0x0000,
                    0x0000,
                    0x0000,
                ),
                vec![brightness],
            ),
            Command::BitmapLinearWin(
                Origin(pixel_x, pixel_y),
                pixels,
                compression,
            ) => {
                debug_assert_eq!(pixel_x % 8, 0);
                debug_assert_eq!(pixels.width() % 8, 0);

                let tile_x = pixel_x / TILE_SIZE;
                let tile_w = pixels.width() as u16 / TILE_SIZE;
                let pixel_h = pixels.height() as u16;
                let payload = into_compressed(compression, pixels.into());
                let command = match compression {
                    CompressionCode::Uncompressed => {
                        CommandCode::BitmapLinearWinUncompressed
                    }
                    CompressionCode::Zlib => CommandCode::BitmapLinearWinZlib,
                    CompressionCode::Bzip2 => CommandCode::BitmapLinearWinBzip2,
                    CompressionCode::Lzma => CommandCode::BitmapLinearWinLzma,
                    CompressionCode::Zstd => CommandCode::BitmapLinearWinZstd,
                };

                Packet(
                    Header(command.into(), tile_x, pixel_y, tile_w, pixel_h),
                    payload,
                )
            }
            Command::BitmapLinear(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinear,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearAnd(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearAnd,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearOr(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearOr,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearXor(offset, bits, compression) => {
                bitmap_linear_into_packet(
                    CommandCode::BitmapLinearXor,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::Cp437Data(Origin(x, y), grid) => Packet(
                Header(
                    CommandCode::Cp437Data.into(),
                    x,
                    y,
                    grid.width() as u16,
                    grid.height() as u16,
                ),
                grid.into(),
            ),
        }
    }
}

#[derive(Debug)]
/// Err values for `Command::try_from`.
#[derive(PartialEq)]
pub enum TryFromPacketError {
    /// the contained command code does not correspond to a known command
    InvalidCommand(u16),
    /// the expected payload size was n, but size m was found
    UnexpectedPayloadSize(usize, usize),
    /// Header fields not needed for the command have been used.
    ///
    /// Note that these commands would usually still work on the actual display.
    ExtraneousHeaderValues,
    /// The contained compression code is not known. This could be of disabled features.
    InvalidCompressionCode(u16),
    /// Decompression of the payload failed. This can be caused by corrupted packets.
    DecompressionFailed,
}

impl TryFrom<Packet> for Command {
    type Error = TryFromPacketError;

    /// Try to interpret the `Packet` as one containing a `Command`
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet(Header(command_u16, a, b, c, d), _) = packet;
        let command_code = match CommandCode::try_from(command_u16) {
            Err(_) => {
                return Err(TryFromPacketError::InvalidCommand(command_u16));
            }
            Ok(value) => value,
        };

        match command_code {
            CommandCode::Clear => match check_command_only(packet) {
                Some(err) => Err(err),
                None => Ok(Command::Clear),
            },
            CommandCode::Brightness => {
                let Packet(header, payload) = packet;
                if payload.len() != 1 {
                    return Err(TryFromPacketError::UnexpectedPayloadSize(
                        1,
                        payload.len(),
                    ));
                }

                let Header(_, a, b, c, d) = header;
                if a != 0 || b != 0 || c != 0 || d != 0 {
                    Err(TryFromPacketError::ExtraneousHeaderValues)
                } else {
                    Ok(Command::Brightness(payload[0]))
                }
            }
            CommandCode::HardReset => match check_command_only(packet) {
                Some(err) => Err(err),
                None => Ok(Command::HardReset),
            },
            CommandCode::FadeOut => match check_command_only(packet) {
                Some(err) => Err(err),
                None => Ok(Command::FadeOut),
            },
            CommandCode::Cp437Data => {
                let Packet(_, payload) = packet;
                Ok(Command::Cp437Data(
                    Origin(a, b),
                    ByteGrid::load(c as usize, d as usize, &payload),
                ))
            }
            CommandCode::CharBrightness => {
                let Packet(_, payload) = packet;
                Ok(Command::CharBrightness(
                    Origin(a, b),
                    ByteGrid::load(c as usize, d as usize, &payload),
                ))
            }
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => Ok(Command::BitmapLegacy),
            CommandCode::BitmapLinear => {
                let (vec, compression) = packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinear(a, vec, compression))
            }
            CommandCode::BitmapLinearAnd => {
                let (vec, compression) = packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearAnd(a, vec, compression))
            }
            CommandCode::BitmapLinearOr => {
                let (vec, compression) = packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearOr(a, vec, compression))
            }
            CommandCode::BitmapLinearXor => {
                let (vec, compression) = packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearXor(a, vec, compression))
            }
            CommandCode::BitmapLinearWinUncompressed => {
                packet_into_bitmap_win(packet, CompressionCode::Uncompressed)
            }
            CommandCode::BitmapLinearWinZlib => {
                packet_into_bitmap_win(packet, CompressionCode::Zlib)
            }
            CommandCode::BitmapLinearWinBzip2 => {
                packet_into_bitmap_win(packet, CompressionCode::Bzip2)
            }
            CommandCode::BitmapLinearWinLzma => {
                packet_into_bitmap_win(packet, CompressionCode::Lzma)
            }
            CommandCode::BitmapLinearWinZstd => {
                packet_into_bitmap_win(packet, CompressionCode::Zstd)
            }
        }
    }
}

fn packet_into_bitmap_win(
    packet: Packet,
    compression: CompressionCode,
) -> Result<Command, TryFromPacketError> {
    let Packet(Header(_, tiles_x, pixels_y, tile_w, pixel_h), payload) = packet;

    let payload = match into_decompressed(compression, payload) {
        None => return Err(TryFromPacketError::DecompressionFailed),
        Some(decompressed) => decompressed,
    };

    Ok(Command::BitmapLinearWin(
        Origin(tiles_x * TILE_SIZE, pixels_y),
        PixelGrid::load(
            tile_w as usize * TILE_SIZE as usize,
            pixel_h as usize,
            &payload,
        ),
        compression,
    ))
}

/// Helper method for BitMapLinear*-Commands into Packet
fn bitmap_linear_into_packet(
    command: CommandCode,
    offset: Offset,
    compression: CompressionCode,
    payload: Vec<u8>,
) -> Packet {
    let length = payload.len() as u16;
    let payload = into_compressed(compression, payload);
    Packet(
        Header(command.into(), offset, length, compression.into(), 0),
        payload,
    )
}

/// Helper method for creating empty packets only containing the command code
fn command_code_only(code: CommandCode) -> Packet {
    Packet(Header(code.into(), 0x0000, 0x0000, 0x0000, 0x0000), vec![])
}

/// Helper method for checking that a packet is empty and only contains a command code
fn check_command_only(packet: Packet) -> Option<TryFromPacketError> {
    let Packet(Header(_, a, b, c, d), payload) = packet;
    if !payload.is_empty() {
        Some(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
    } else if a != 0 || b != 0 || c != 0 || d != 0 {
        Some(TryFromPacketError::ExtraneousHeaderValues)
    } else {
        None
    }
}

/// Helper method for Packets into BitMapLinear*-Commands
fn packet_into_linear_bitmap(
    packet: Packet,
) -> Result<(BitVec, CompressionCode), TryFromPacketError> {
    let Packet(Header(_, _, length, sub, reserved), payload) = packet;
    if reserved != 0 {
        return Err(TryFromPacketError::ExtraneousHeaderValues);
    }
    let sub = match CompressionCode::try_from(sub) {
        Err(_) => return Err(TryFromPacketError::InvalidCompressionCode(sub)),
        Ok(value) => value,
    };
    let payload = match into_decompressed(sub, payload) {
        None => return Err(TryFromPacketError::DecompressionFailed),
        Some(value) => value,
    };
    if payload.len() != length as usize {
        return Err(TryFromPacketError::UnexpectedPayloadSize(
            length as usize,
            payload.len(),
        ));
    }
    Ok((BitVec::from(&*payload), sub))
}

#[cfg(feature = "c_api")]
pub mod c_api {
    use std::ptr::null_mut;

    use crate::{
        BitVec, Brightness, ByteGrid, Command, CompressionCode, Offset, Origin,
        Packet, PixelGrid,
    };

    /// Tries to turn a `Packet` into a `Command`. The packet is gets deallocated in the process.
    ///
    /// Returns: pointer to command or NULL
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_try_from_packet(
        packet: *mut Packet,
    ) -> *mut Command {
        let packet = *Box::from_raw(packet);
        match Command::try_from(packet) {
            Err(_) => null_mut(),
            Ok(command) => Box::into_raw(Box::new(command)),
        }
    }

    /// Clones a `Command` instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_clone(
        original: *const Command,
    ) -> *mut Command {
        Box::into_raw(Box::new((*original).clone()))
    }

    /// Allocates a new `Command::Clear` instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_clear() -> *mut Command {
        Box::into_raw(Box::new(Command::Clear))
    }

    /// Allocates a new `Command::HardReset` instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_hard_reset() -> *mut Command {
        Box::into_raw(Box::new(Command::HardReset))
    }

    /// Allocates a new `Command::FadeOut` instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_fade_out() -> *mut Command {
        Box::into_raw(Box::new(Command::FadeOut))
    }

    /// Allocates a new `Command::Brightness` instance
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_brightness(
        brightness: Brightness,
    ) -> *mut Command {
        Box::into_raw(Box::new(Command::Brightness(brightness)))
    }

    /// Allocates a new `Command::CharBrightness` instance.
    /// The passed `ByteGrid` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_char_brightness(
        x: u16,
        y: u16,
        byte_grid: *mut ByteGrid,
    ) -> *mut Command {
        let byte_grid = *Box::from_raw(byte_grid);
        Box::into_raw(Box::new(Command::CharBrightness(
            Origin(x, y),
            byte_grid,
        )))
    }

    /// Allocates a new `Command::BitmapLinear` instance.
    /// The passed `BitVec` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_bitmap_linear(
        offset: Offset,
        bit_vec: *mut BitVec,
        compression: CompressionCode,
    ) -> *mut Command {
        let bit_vec = *Box::from_raw(bit_vec);
        Box::into_raw(Box::new(Command::BitmapLinear(
            offset,
            bit_vec,
            compression,
        )))
    }

    /// Allocates a new `Command::BitmapLinearAnd` instance.
    /// The passed `BitVec` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_bitmap_linear_and(
        offset: Offset,
        bit_vec: *mut BitVec,
        compression: CompressionCode,
    ) -> *mut Command {
        let bit_vec = *Box::from_raw(bit_vec);
        Box::into_raw(Box::new(Command::BitmapLinearAnd(
            offset,
            bit_vec,
            compression,
        )))
    }

    /// Allocates a new `Command::BitmapLinearOr` instance.
    /// The passed `BitVec` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_bitmap_linear_or(
        offset: Offset,
        bit_vec: *mut BitVec,
        compression: CompressionCode,
    ) -> *mut Command {
        let bit_vec = *Box::from_raw(bit_vec);
        Box::into_raw(Box::new(Command::BitmapLinearOr(
            offset,
            bit_vec,
            compression,
        )))
    }

    /// Allocates a new `Command::BitmapLinearXor` instance.
    /// The passed `BitVec` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_bitmap_linear_xor(
        offset: Offset,
        bit_vec: *mut BitVec,
        compression: CompressionCode,
    ) -> *mut Command {
        let bit_vec = *Box::from_raw(bit_vec);
        Box::into_raw(Box::new(Command::BitmapLinearXor(
            offset,
            bit_vec,
            compression,
        )))
    }

    /// Allocates a new `Command::Cp437Data` instance.
    /// The passed `ByteGrid` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_cp437_data(
        x: u16,
        y: u16,
        byte_grid: *mut ByteGrid,
    ) -> *mut Command {
        let byte_grid = *Box::from_raw(byte_grid);
        Box::into_raw(Box::new(Command::Cp437Data(Origin(x, y), byte_grid)))
    }

    /// Allocates a new `Command::BitmapLinearWin` instance.
    /// The passed `PixelGrid` gets deallocated in the process.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_bitmap_linear_win(
        x: u16,
        y: u16,
        byte_grid: *mut PixelGrid,
        compression_code: CompressionCode,
    ) -> *mut Command {
        let byte_grid = *Box::from_raw(byte_grid);
        Box::into_raw(Box::new(Command::BitmapLinearWin(
            Origin(x, y),
            byte_grid,
            compression_code,
        )))
    }

    /// Deallocates a `Command`. Note that connection_send does this implicitly, so you only need
    /// to do this if you use the library for parsing commands.
    #[no_mangle]
    pub unsafe extern "C" fn sp2_command_dealloc(ptr: *mut Command) {
        _ = Box::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    use crate::command::TryFromPacketError;
    use crate::command_code::CommandCode;
    use crate::{
        BitVec, ByteGrid, Command, CompressionCode, Header, Origin, Packet,
        PixelGrid,
    };

    fn round_trip(original: Command) {
        let packet: Packet = original.clone().into();
        let copy: Command = match Command::try_from(packet) {
            Ok(command) => command,
            Err(err) => panic!("could not reload {original:?}: {err:?}"),
        };
        assert_eq!(copy, original);
    }

    fn all_compressions() -> [CompressionCode; 5] {
        [
            CompressionCode::Uncompressed,
            CompressionCode::Lzma,
            CompressionCode::Bzip2,
            CompressionCode::Zlib,
            CompressionCode::Zstd,
        ]
    }

    #[test]
    fn round_trip_clear() {
        round_trip(Command::Clear);
    }

    #[test]
    fn round_trip_hard_reset() {
        round_trip(Command::HardReset);
    }

    #[test]
    fn round_trip_fade_out() {
        round_trip(Command::FadeOut);
    }

    #[test]
    fn round_trip_brightness() {
        round_trip(Command::Brightness(6));
    }

    #[test]
    #[allow(deprecated)]
    fn round_trip_bitmap_legacy() {
        round_trip(Command::BitmapLegacy);
    }

    #[test]
    fn round_trip_char_brightness() {
        round_trip(Command::CharBrightness(Origin(5, 2), ByteGrid::new(7, 5)));
    }

    #[test]
    fn round_trip_cp437_data() {
        round_trip(Command::Cp437Data(Origin(5, 2), ByteGrid::new(7, 5)));
    }

    #[test]
    fn round_trip_bitmap_linear() {
        for compression in all_compressions() {
            round_trip(Command::BitmapLinear(23, BitVec::new(40), compression));
            round_trip(Command::BitmapLinearAnd(
                23,
                BitVec::new(40),
                compression,
            ));
            round_trip(Command::BitmapLinearOr(
                23,
                BitVec::new(40),
                compression,
            ));
            round_trip(Command::BitmapLinearXor(
                23,
                BitVec::new(40),
                compression,
            ));
            round_trip(Command::BitmapLinearWin(
                Origin(0, 0),
                PixelGrid::max_sized(),
                compression,
            ));
        }
    }

    #[test]
    fn error_invalid_command() {
        let p = Packet(Header(0xFF, 0x00, 0x00, 0x00, 0x00), vec![]);
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::InvalidCommand(0xFF))
        ))
    }

    #[test]
    fn error_extraneous_header_values_clear() {
        let p = Packet(
            Header(CommandCode::Clear.into(), 0x05, 0x00, 0x00, 0x00),
            vec![],
        );
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_values_brightness() {
        let p = Packet(
            Header(CommandCode::Brightness.into(), 0x00, 0x13, 0x37, 0x00),
            vec![5],
        );
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_hard_reset() {
        let p = Packet(
            Header(CommandCode::HardReset.into(), 0x00, 0x00, 0x00, 0x01),
            vec![],
        );
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_fade_out() {
        let p = Packet(
            Header(CommandCode::FadeOut.into(), 0x10, 0x00, 0x00, 0x01),
            vec![],
        );
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_unexpected_payload() {
        let p = Packet(
            Header(CommandCode::FadeOut.into(), 0x00, 0x00, 0x00, 0x00),
            vec![5, 7],
        );
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::UnexpectedPayloadSize(0, 2))
        ))
    }

    #[test]
    fn error_decompression_failed_win() {
        for compression in all_compressions() {
            let p: Packet = Command::BitmapLinearWin(
                Origin(16, 8),
                PixelGrid::new(8, 8),
                compression,
            )
            .into();
            let Packet(header, mut payload) = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet(header, payload);
            let result = Command::try_from(p);
            if compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn error_decompression_failed_and() {
        for compression in all_compressions() {
            let p: Packet =
                Command::BitmapLinearAnd(0, BitVec::new(8), compression).into();
            let Packet(header, mut payload) = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet(header, payload);
            let result = Command::try_from(p);
            if compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                // when not compressing, there is no way to detect corrupted data
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn unexpected_payload_size_brightness() {
        assert_eq!(
            Command::try_from(Packet(
                Header(CommandCode::Brightness.into(), 0, 0, 0, 0),
                vec!()
            )),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 0))
        );

        assert_eq!(
            Command::try_from(Packet(
                Header(CommandCode::Brightness.into(), 0, 0, 0, 0),
                vec!(0, 0)
            )),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 2))
        );
    }

    #[test]
    fn error_reserved_used() {
        let Packet(header, payload) = Command::BitmapLinear(
            0,
            BitVec::new(8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header(command, offset, length, sub, _reserved) = header;
        let p = Packet(Header(command, offset, length, sub, 69), payload);
        assert_eq!(
            Command::try_from(p),
            Err(TryFromPacketError::ExtraneousHeaderValues)
        );
    }

    #[test]
    fn error_invalid_compression() {
        let Packet(header, payload) = Command::BitmapLinear(
            0,
            BitVec::new(8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header(command, offset, length, _sub, reserved) = header;
        let p = Packet(Header(command, offset, length, 42, reserved), payload);
        assert_eq!(
            Command::try_from(p),
            Err(TryFromPacketError::InvalidCompressionCode(42))
        );
    }

    #[test]
    fn error_unexpected_size() {
        let Packet(header, payload) = Command::BitmapLinear(
            0,
            BitVec::new(8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header(command, offset, length, compression, reserved) = header;
        let p = Packet(
            Header(command, offset, 420, compression, reserved),
            payload,
        );
        assert_eq!(
            Command::try_from(p),
            Err(TryFromPacketError::UnexpectedPayloadSize(
                420,
                length as usize
            ))
        );
    }
}
