use bitvec::prelude::BitVec;

use crate::{
    command_code::CommandCode,
    compression::{into_compressed, into_decompressed},
    Brightness, BrightnessGrid, CompressionCode, Grid, Header, Origin, Packet,
    PixelGrid, Pixels, PrimitiveGrid, SpBitVec, Tiles, TILE_SIZE,
};

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = usize;

/// A grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
pub type Cp473Grid = PrimitiveGrid<u8>;

/// A command to send to the display.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Set all pixels to the off state. Does not affect brightness.
    Clear,

    /// Show text on the screen.
    ///
    /// <div class="warning">
    ///     The library does not currently convert between UTF-8 and CP-437.
    ///     Because Rust expects UTF-8 strings, it might be necessary to only send ASCII for now.
    /// </div>
    Cp437Data(Origin<Tiles>, Cp473Grid),

    /// Sets a window of pixels to the specified values
    BitmapLinearWin(Origin<Pixels>, PixelGrid, CompressionCode),

    /// Set the brightness of all tiles to the same value.
    Brightness(Brightness),

    /// Set the brightness of individual tiles in a rectangular area of the display.
    CharBrightness(Origin<Tiles>, BrightnessGrid),

    /// Set pixel data starting at the pixel offset on screen.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained `BitVec` is always uncompressed.
    BitmapLinear(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to an and-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained `BitVec` is always uncompressed.
    BitmapLinearAnd(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to an or-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained `BitVec` is always uncompressed.
    BitmapLinearOr(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to a xor-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained `BitVec` is always uncompressed.
    BitmapLinearXor(Offset, SpBitVec, CompressionCode),

    /// Kills the udp daemon on the display, which usually results in a restart.
    ///
    /// Please do not send this in your normal program flow.
    HardReset,

    /// <div class="warning">Untested</div>
    ///
    /// Slowly decrease brightness until off or something like that?
    FadeOut,

    #[deprecated]
    /// Legacy command code, gets ignored by the real display.
    ///
    /// Might be useful as a noop package.
    BitmapLegacy,
}

impl From<Command> for Packet {
    /// Move the `Command` into a `Packet` instance for sending.
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: Command) -> Self {
        match value {
            Command::Clear => Command::command_code_only(CommandCode::Clear),
            Command::FadeOut => {
                Command::command_code_only(CommandCode::FadeOut)
            }
            Command::HardReset => {
                Command::command_code_only(CommandCode::HardReset)
            }
            #[allow(deprecated)]
            Command::BitmapLegacy => {
                Command::command_code_only(CommandCode::BitmapLegacy)
            }
            Command::CharBrightness(origin, grid) => Packet(
                Header(
                    CommandCode::CharBrightness.into(),
                    origin.x as u16,
                    origin.y as u16,
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
                vec![brightness.into()],
            ),
            Command::BitmapLinearWin(origin, pixels, compression) => {
                bitmap_win_into_packet(origin, pixels, compression)
            }
            Command::BitmapLinear(offset, bits, compression) => {
                Command::bitmap_linear_into_packet(
                    CommandCode::BitmapLinear,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearAnd(offset, bits, compression) => {
                Command::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearAnd,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearOr(offset, bits, compression) => {
                Command::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearOr,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::BitmapLinearXor(offset, bits, compression) => {
                Command::bitmap_linear_into_packet(
                    CommandCode::BitmapLinearXor,
                    offset,
                    compression,
                    bits.into(),
                )
            }
            Command::Cp437Data(origin, grid) => Packet(
                Header(
                    CommandCode::Cp437Data.into(),
                    origin.x as u16,
                    origin.y as u16,
                    grid.width() as u16,
                    grid.height() as u16,
                ),
                grid.into(),
            ),
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn bitmap_win_into_packet(
    origin: Origin<Pixels>,
    pixels: PixelGrid,
    compression: CompressionCode,
) -> Packet {
    debug_assert_eq!(origin.x % 8, 0);
    debug_assert_eq!(pixels.width() % 8, 0);

    let tile_x = (origin.x / TILE_SIZE) as u16;
    let tile_w = (pixels.width() / TILE_SIZE) as u16;
    let pixel_h = pixels.height() as u16;
    let payload = into_compressed(compression, pixels.into());
    let command = match compression {
        CompressionCode::Uncompressed => {
            CommandCode::BitmapLinearWinUncompressed
        }
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => CommandCode::BitmapLinearWinZlib,
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => CommandCode::BitmapLinearWinBzip2,
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => CommandCode::BitmapLinearWinLzma,
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => CommandCode::BitmapLinearWinZstd,
    };

    Packet(
        Header(command.into(), tile_x, origin.y as u16, tile_w, pixel_h),
        payload,
    )
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
    /// The given brightness value is out of bounds
    InvalidBrightness(u8),
}

impl TryFrom<Packet> for Command {
    type Error = TryFromPacketError;

    /// Try to interpret the `Packet` as one containing a `Command`
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet(Header(command_u16, a, b, c, d), _) = packet;
        let command_code = match CommandCode::try_from(command_u16) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCommand(command_u16));
            }
            Ok(value) => value,
        };

        match command_code {
            CommandCode::Clear => match Self::check_command_only(packet) {
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
                    return Err(TryFromPacketError::ExtraneousHeaderValues);
                }

                match Brightness::try_from(payload[0]) {
                    Ok(b) => Ok(Command::Brightness(b)),
                    Err(_) => {
                        Err(TryFromPacketError::InvalidBrightness(payload[0]))
                    }
                }
            }
            CommandCode::HardReset => match Self::check_command_only(packet) {
                Some(err) => Err(err),
                None => Ok(Command::HardReset),
            },
            CommandCode::FadeOut => match Self::check_command_only(packet) {
                Some(err) => Err(err),
                None => Ok(Command::FadeOut),
            },
            CommandCode::Cp437Data => {
                let Packet(_, payload) = packet;
                Ok(Command::Cp437Data(
                    Origin::new(a as usize, b as usize),
                    Cp473Grid::load(c as usize, d as usize, &payload),
                ))
            }
            CommandCode::CharBrightness => {
                let Packet(_, payload) = packet;

                let grid =
                    PrimitiveGrid::load(c as usize, d as usize, &payload);
                let grid = match BrightnessGrid::try_from(grid) {
                    Ok(grid) => grid,
                    Err(val) => {
                        return Err(TryFromPacketError::InvalidBrightness(val))
                    }
                };

                Ok(Command::CharBrightness(
                    Origin::new(a as usize, b as usize),
                    grid,
                ))
            }
            #[allow(deprecated)]
            CommandCode::BitmapLegacy => Ok(Command::BitmapLegacy),
            CommandCode::BitmapLinear => {
                let (vec, compression) =
                    Self::packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinear(a as Offset, vec, compression))
            }
            CommandCode::BitmapLinearAnd => {
                let (vec, compression) =
                    Self::packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearAnd(a as Offset, vec, compression))
            }
            CommandCode::BitmapLinearOr => {
                let (vec, compression) =
                    Self::packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearOr(a as Offset, vec, compression))
            }
            CommandCode::BitmapLinearXor => {
                let (vec, compression) =
                    Self::packet_into_linear_bitmap(packet)?;
                Ok(Command::BitmapLinearXor(a as Offset, vec, compression))
            }
            CommandCode::BitmapLinearWinUncompressed => {
                Self::packet_into_bitmap_win(
                    packet,
                    CompressionCode::Uncompressed,
                )
            }
            #[cfg(feature = "compression_zlib")]
            CommandCode::BitmapLinearWinZlib => {
                Self::packet_into_bitmap_win(packet, CompressionCode::Zlib)
            }
            #[cfg(feature = "compression_bzip2")]
            CommandCode::BitmapLinearWinBzip2 => {
                Self::packet_into_bitmap_win(packet, CompressionCode::Bzip2)
            }
            #[cfg(feature = "compression_lzma")]
            CommandCode::BitmapLinearWinLzma => {
                Self::packet_into_bitmap_win(packet, CompressionCode::Lzma)
            }
            #[cfg(feature = "compression_zstd")]
            CommandCode::BitmapLinearWinZstd => {
                Self::packet_into_bitmap_win(packet, CompressionCode::Zstd)
            }
        }
    }
}

impl Command {
    fn packet_into_bitmap_win(
        packet: Packet,
        compression: CompressionCode,
    ) -> Result<Command, TryFromPacketError> {
        let Packet(Header(_, tiles_x, pixels_y, tile_w, pixel_h), payload) =
            packet;

        let payload = match into_decompressed(compression, payload) {
            None => return Err(TryFromPacketError::DecompressionFailed),
            Some(decompressed) => decompressed,
        };

        Ok(Command::BitmapLinearWin(
            Origin::new(tiles_x as usize * TILE_SIZE, pixels_y as usize),
            PixelGrid::load(
                tile_w as usize * TILE_SIZE,
                pixel_h as usize,
                &payload,
            ),
            compression,
        ))
    }

    /// Helper method for `BitMapLinear*`-Commands into `Packet`
    #[allow(clippy::cast_possible_truncation)]
    fn bitmap_linear_into_packet(
        command: CommandCode,
        offset: Offset,
        compression: CompressionCode,
        payload: Vec<u8>,
    ) -> Packet {
        let length = payload.len() as u16;
        let payload = into_compressed(compression, payload);
        Packet(
            Header(
                command.into(),
                offset as u16,
                length,
                compression.into(),
                0,
            ),
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

    /// Helper method for Packets into `BitMapLinear*`-Commands
    fn packet_into_linear_bitmap(
        packet: Packet,
    ) -> Result<(SpBitVec, CompressionCode), TryFromPacketError> {
        let Packet(Header(_, _, length, sub, reserved), payload) = packet;
        if reserved != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }
        let sub = match CompressionCode::try_from(sub) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCompressionCode(sub));
            }
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
        Ok((BitVec::from_vec(payload), sub))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitvec::prelude::BitVec, command::TryFromPacketError,
        command_code::CommandCode, origin::Pixels, Brightness, Command,
        CompressionCode, Header, Origin, Packet, PixelGrid, PrimitiveGrid,
    };

    fn round_trip(original: Command) {
        let packet: Packet = original.clone().into();
        let copy: Command = match Command::try_from(packet) {
            Ok(command) => command,
            Err(err) => panic!("could not reload {original:?}: {err:?}"),
        };
        assert_eq!(copy, original);
    }

    fn all_compressions<'t>() -> &'t [CompressionCode] {
        &[
            CompressionCode::Uncompressed,
            #[cfg(feature = "compression_lzma")]
            CompressionCode::Lzma,
            #[cfg(feature = "compression_bzip2")]
            CompressionCode::Bzip2,
            #[cfg(feature = "compression_zlib")]
            CompressionCode::Zlib,
            #[cfg(feature = "compression_zstd")]
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
        round_trip(Command::Brightness(Brightness::try_from(6).unwrap()));
    }

    #[test]
    #[allow(deprecated)]
    fn round_trip_bitmap_legacy() {
        round_trip(Command::BitmapLegacy);
    }

    #[test]
    fn round_trip_char_brightness() {
        round_trip(Command::CharBrightness(
            Origin::new(5, 2),
            PrimitiveGrid::new(7, 5),
        ));
    }

    #[test]
    fn round_trip_cp437_data() {
        round_trip(Command::Cp437Data(
            Origin::new(5, 2),
            PrimitiveGrid::new(7, 5),
        ));
    }

    #[test]
    fn round_trip_bitmap_linear() {
        for compression in all_compressions().to_owned() {
            round_trip(Command::BitmapLinear(
                23,
                BitVec::repeat(false, 40),
                compression,
            ));
            round_trip(Command::BitmapLinearAnd(
                23,
                BitVec::repeat(false, 40),
                compression,
            ));
            round_trip(Command::BitmapLinearOr(
                23,
                BitVec::repeat(false, 40),
                compression,
            ));
            round_trip(Command::BitmapLinearXor(
                23,
                BitVec::repeat(false, 40),
                compression,
            ));
            round_trip(Command::BitmapLinearWin(
                Origin::new(0, 0),
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
        for compression in all_compressions().to_owned() {
            let p: Packet = Command::BitmapLinearWin(
                Origin::new(16, 8),
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
        for compression in all_compressions().to_owned() {
            let p: Packet = Command::BitmapLinearAnd(
                0,
                BitVec::repeat(false, 8),
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
                vec!(),
            )),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 0))
        );

        assert_eq!(
            Command::try_from(Packet(
                Header(CommandCode::Brightness.into(), 0, 0, 0, 0),
                vec!(0, 0),
            )),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 2))
        );
    }

    #[test]
    fn error_reserved_used() {
        let Packet(header, payload) = Command::BitmapLinear(
            0,
            BitVec::repeat(false, 8),
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
            BitVec::repeat(false, 8),
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
            BitVec::repeat(false, 8),
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
                length as usize,
            ))
        );
    }

    #[test]
    fn origin_add() {
        assert_eq!(
            Origin::<Pixels>::new(4, 2),
            Origin::new(1, 0) + Origin::new(3, 2)
        );
    }
}
