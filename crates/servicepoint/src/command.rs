use bitvec::prelude::BitVec;

use crate::{
    command_code::CommandCode,
    compression::into_decompressed,
    packet::{Header, Packet},
    Brightness, BrightnessGrid, CompressionCode, Origin, PixelGrid, Pixels,
    PrimitiveGrid, SpBitVec, Tiles, TILE_SIZE,
};

/// Type alias for documenting the meaning of the u16 in enum values
pub type Offset = usize;

/// A grid containing codepage 437 characters.
///
/// The encoding is currently not enforced.
pub type Cp437Grid = PrimitiveGrid<u8>;

/// A low-level display command.
///
/// This struct and associated functions implement the UDP protocol for the display.
///
/// To send a [Command], use a [connection][crate::Connection].
///
/// # Available commands
///
/// To send text, take a look at [Command::Cp437Data].
///
/// To draw pixels, the easiest command to use is [Command::BitmapLinearWin].
///
/// The other BitmapLinear-Commands operate on a region of pixel memory directly.
/// [Command::BitmapLinear] overwrites a region.
/// [Command::BitmapLinearOr], [Command::BitmapLinearAnd] and [Command::BitmapLinearXor] apply logical operations per pixel.
///
/// Out of bounds operations may be truncated or ignored by the display.
///
/// # Compression
///
/// Some commands can contain compressed payloads.
/// To get started, use [CompressionCode::Uncompressed].
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
/// # use servicepoint::{Brightness, Command, Connection, packet::Packet};
/// #
/// // create command
/// let command = Command::Brightness(Brightness::MAX);
///
/// // turn command into Packet
/// let packet: Packet = command.clone().into();
///
/// // read command from packet
/// let round_tripped = Command::try_from(packet).unwrap();
///
/// // round tripping produces exact copy
/// assert_eq!(command, round_tripped);
///
/// // send command
/// # let connection = Connection::open("127.0.0.1:2342").unwrap();
/// connection.send(command).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Set all pixels to the off state. Does not affect brightness.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// connection.send(Command::Clear).unwrap();
    /// ```
    Clear,

    /// Show text on the screen.
    ///
    /// The text is sent in the form of a 2D grid of characters.
    ///
    /// <div class="warning">
    ///     The library does not currently convert between UTF-8 and CP-437.
    ///     Because Rust expects UTF-8 strings, it might be necessary to only send ASCII for now.
    /// </div>
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection, Cp437Grid, Origin};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// let chars = ['H', 'e', 'l', 'l', 'o', 'W', 'o', 'r', 'l', 'd'].map(move |c| c as u8);
    /// let grid = Cp437Grid::load(5, 2, &chars);
    /// connection.send(Command::Cp437Data(Origin::new(2, 2), grid)).unwrap();
    /// ```
    Cp437Data(Origin<Tiles>, Cp437Grid),

    /// Overwrites a rectangular region of pixels.
    ///
    /// Origin coordinates must be divisible by 8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, CompressionCode, Grid, PixelGrid};
    /// # let connection = servicepoint::Connection::Fake;
    /// #
    /// let mut pixels = PixelGrid::max_sized();
    /// // draw something to the pixels here
    /// # pixels.set(2, 5, true);
    ///
    /// // create command to send pixels
    /// let command = Command::BitmapLinearWin(
    ///    servicepoint::Origin::new(0, 0),
    ///    pixels,
    ///    CompressionCode::Uncompressed
    /// );
    ///
    /// connection.send(command).expect("send failed");
    /// ```
    BitmapLinearWin(Origin<Pixels>, PixelGrid, CompressionCode),

    /// Set the brightness of all tiles to the same value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Brightness, Command, Connection};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// let command = Command::Brightness(Brightness::MAX);
    /// connection.send(command).unwrap();
    /// ```
    Brightness(Brightness),

    /// Set the brightness of individual tiles in a rectangular area of the display.
    CharBrightness(Origin<Tiles>, BrightnessGrid),

    /// Set pixel data starting at the pixel offset on screen.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained [BitVec] is always uncompressed.
    BitmapLinear(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to an and-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained [BitVec] is always uncompressed.
    BitmapLinearAnd(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to an or-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained [BitVec] is always uncompressed.
    BitmapLinearOr(Offset, SpBitVec, CompressionCode),

    /// Set pixel data according to a xor-mask starting at the offset.
    ///
    /// The screen will continuously overwrite more pixel data without regarding the offset, meaning
    /// once the starting row is full, overwriting will continue on column 0.
    ///
    /// The contained [BitVec] is always uncompressed.
    BitmapLinearXor(Offset, SpBitVec, CompressionCode),

    /// Kills the udp daemon on the display, which usually results in a restart.
    ///
    /// Please do not send this in your normal program flow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// connection.send(Command::HardReset).unwrap();
    /// ```
    HardReset,

    /// <div class="warning">Untested</div>
    ///
    /// Slowly decrease brightness until off or something like that?
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// connection.send(Command::FadeOut).unwrap();
    /// ```
    FadeOut,

    /// Legacy command code, gets ignored by the real display.
    ///
    /// Might be useful as a noop package.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use servicepoint::{Command, Connection};
    /// # let connection = Connection::open("127.0.0.1:2342").unwrap();
    /// // this sends a packet that does nothing
    /// # #[allow(deprecated)]
    /// connection.send(Command::BitmapLegacy).unwrap();
    /// ```
    #[deprecated]
    BitmapLegacy,
}

#[derive(Debug)]
/// Err values for [Command::try_from].
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

    /// Try to interpret the [Packet] as one containing a [Command]
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let Packet {
            header: Header {
                command_code, a, ..
            },
            ..
        } = packet;
        let command_code = match CommandCode::try_from(command_code) {
            Err(()) => {
                return Err(TryFromPacketError::InvalidCommand(command_code));
            }
            Ok(value) => value,
        };

        match command_code {
            CommandCode::Clear => {
                Self::packet_into_command_only(packet, Command::Clear)
            }
            CommandCode::Brightness => Self::packet_into_brightness(&packet),
            CommandCode::HardReset => {
                Self::packet_into_command_only(packet, Command::HardReset)
            }
            CommandCode::FadeOut => {
                Self::packet_into_command_only(packet, Command::FadeOut)
            }
            CommandCode::Cp437Data => Self::packet_into_cp437(&packet),
            CommandCode::CharBrightness => {
                Self::packet_into_char_brightness(&packet)
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
        let Packet {
            header:
                Header {
                    command_code: _,
                    a: tiles_x,
                    b: pixels_y,
                    c: tile_w,
                    d: pixel_h,
                },
            payload,
        } = packet;

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

    /// Helper method for checking that a packet is empty and only contains a command code
    fn packet_into_command_only(
        packet: Packet,
        command: Command,
    ) -> Result<Command, TryFromPacketError> {
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
        if !payload.is_empty() {
            Err(TryFromPacketError::UnexpectedPayloadSize(0, payload.len()))
        } else if a != 0 || b != 0 || c != 0 || d != 0 {
            Err(TryFromPacketError::ExtraneousHeaderValues)
        } else {
            Ok(command)
        }
    }

    /// Helper method for Packets into `BitMapLinear*`-Commands
    fn packet_into_linear_bitmap(
        packet: Packet,
    ) -> Result<(SpBitVec, CompressionCode), TryFromPacketError> {
        let Packet {
            header:
                Header {
                    b: length,
                    c: sub,
                    d: reserved,
                    ..
                },
            payload,
        } = packet;
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

    fn packet_into_char_brightness(
        packet: &Packet,
    ) -> Result<Command, TryFromPacketError> {
        let Packet {
            header:
                Header {
                    command_code: _,
                    a: x,
                    b: y,
                    c: width,
                    d: height,
                },
            payload,
        } = packet;

        let grid =
            PrimitiveGrid::load(*width as usize, *height as usize, payload);
        let grid = match BrightnessGrid::try_from(grid) {
            Ok(grid) => grid,
            Err(val) => return Err(TryFromPacketError::InvalidBrightness(val)),
        };

        Ok(Command::CharBrightness(
            Origin::new(*x as usize, *y as usize),
            grid,
        ))
    }

    fn packet_into_brightness(
        packet: &Packet,
    ) -> Result<Command, TryFromPacketError> {
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
        if payload.len() != 1 {
            return Err(TryFromPacketError::UnexpectedPayloadSize(
                1,
                payload.len(),
            ));
        }

        if *a != 0 || *b != 0 || *c != 0 || *d != 0 {
            return Err(TryFromPacketError::ExtraneousHeaderValues);
        }

        match Brightness::try_from(payload[0]) {
            Ok(b) => Ok(Command::Brightness(b)),
            Err(_) => Err(TryFromPacketError::InvalidBrightness(payload[0])),
        }
    }

    fn packet_into_cp437(
        packet: &Packet,
    ) -> Result<Command, TryFromPacketError> {
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
        Ok(Command::Cp437Data(
            Origin::new(*a as usize, *b as usize),
            Cp437Grid::load(*c as usize, *d as usize, payload),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitvec::prelude::BitVec,
        command::TryFromPacketError,
        command_code::CommandCode,
        origin::Pixels,
        packet::{Header, Packet},
        Brightness, Command, CompressionCode, Origin, PixelGrid, PrimitiveGrid,
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
        let p = Packet {
            header: Header {
                command_code: 0xFF,
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![],
        };
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::InvalidCommand(0xFF))
        ))
    }

    #[test]
    fn error_extraneous_header_values_clear() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Clear.into(),
                a: 0x05,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![],
        };
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_values_brightness() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::Brightness.into(),
                a: 0x00,
                b: 0x13,
                c: 0x37,
                d: 0x00,
            },
            payload: vec![5],
        };
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_hard_reset() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::HardReset.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x01,
            },
            payload: vec![],
        };
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_extraneous_header_fade_out() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x10,
                b: 0x00,
                c: 0x00,
                d: 0x01,
            },
            payload: vec![],
        };
        let result = Command::try_from(p);
        assert!(matches!(
            result,
            Err(TryFromPacketError::ExtraneousHeaderValues)
        ))
    }

    #[test]
    fn error_unexpected_payload() {
        let p = Packet {
            header: Header {
                command_code: CommandCode::FadeOut.into(),
                a: 0x00,
                b: 0x00,
                c: 0x00,
                d: 0x00,
            },
            payload: vec![5, 7],
        };
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

            let Packet {
                header,
                mut payload,
            } = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet { header, payload };
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
            let Packet {
                header,
                mut payload,
            } = p;

            // mangle it
            for byte in payload.iter_mut() {
                *byte -= *byte / 2;
            }

            let p = Packet { header, payload };
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
            Command::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!()
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 0))
        );

        assert_eq!(
            Command::try_from(Packet {
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
                payload: vec!(0, 0)
            }),
            Err(TryFromPacketError::UnexpectedPayloadSize(1, 2))
        );
    }

    #[test]
    fn error_reserved_used() {
        let Packet { header, payload } = Command::BitmapLinear(
            0,
            BitVec::repeat(false, 8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: sub,
            d: _reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: length,
                c: sub,
                d: 69,
            },
            payload,
        };
        assert_eq!(
            Command::try_from(p),
            Err(TryFromPacketError::ExtraneousHeaderValues)
        );
    }

    #[test]
    fn error_invalid_compression() {
        let Packet { header, payload } = Command::BitmapLinear(
            0,
            BitVec::repeat(false, 8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: _sub,
            d: reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: length,
                c: 42,
                d: reserved,
            },
            payload,
        };
        assert_eq!(
            Command::try_from(p),
            Err(TryFromPacketError::InvalidCompressionCode(42))
        );
    }

    #[test]
    fn error_unexpected_size() {
        let Packet { header, payload } = Command::BitmapLinear(
            0,
            BitVec::repeat(false, 8),
            CompressionCode::Uncompressed,
        )
        .into();
        let Header {
            command_code: command,
            a: offset,
            b: length,
            c: compression,
            d: reserved,
        } = header;
        let p = Packet {
            header: Header {
                command_code: command,
                a: offset,
                b: 420,
                c: compression,
                d: reserved,
            },
            payload,
        };
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
