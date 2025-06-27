use crate::{
    command_code::{CommandCode, InvalidCommandCodeError},
    commands::errors::{TryFromPacketError, TryIntoPacketError},
    compression::{compress, decompress, CompressionError},
    Bitmap, CompressionCode, DataRef, Grid, Header, Origin, Packet, Pixels,
    TypedCommand, TILE_SIZE,
};

/// Overwrites a rectangular region of pixels.
///
/// Origin coordinates must be divisible by 8.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// # let connection = FakeConnection;
/// #
/// let mut bitmap = Bitmap::max_sized();
/// // draw something to the pixels here
/// # bitmap.set(2, 5, true);
///
/// // create command to send pixels
/// let command = BitmapCommand {
///     bitmap,
///     origin: Origin::ZERO,
///     compression: CompressionCode::Uncompressed
/// };
///
/// connection.send_command(command).expect("send failed");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitmapCommand {
    /// the pixels to send
    pub bitmap: Bitmap,
    /// where to start drawing the pixels
    pub origin: Origin<Pixels>,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl TryFrom<&BitmapCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: &BitmapCommand) -> Result<Self, Self::Error> {
        let tile_x = (value.origin.x / TILE_SIZE).try_into()?;
        let tile_w = (value.bitmap.width() / TILE_SIZE).try_into()?;
        let pixel_h = value.bitmap.height().try_into()?;
        let command =
            BitmapCommand::command_code_for_compression(value.compression);
        let data_ref = value.bitmap.data_ref();
        let payload = match compress(value.compression, data_ref) {
            Ok(payload) => payload,
            Err(CompressionError::NoCompression) => data_ref.to_vec(),
            Err(_) => return Err(TryIntoPacketError::CompressionFailed),
        };

        Ok(Packet {
            header: Header {
                command_code: command.into(),
                a: tile_x,
                b: value.origin.y.try_into()?,
                c: tile_w,
                d: pixel_h,
            },
            payload: Some(payload),
        })
    }
}

impl TryFrom<BitmapCommand> for Packet {
    type Error = TryIntoPacketError;

    fn try_from(value: BitmapCommand) -> Result<Self, Self::Error> {
        Packet::try_from(&value)
    }
}

impl TryFrom<Packet> for BitmapCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let code = CommandCode::try_from(packet.header.command_code)?;
        let compression = BitmapCommand::compression_for_command_code(code)
            .ok_or(InvalidCommandCodeError(packet.header.command_code))?;

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

        let expected = tile_w as usize * pixel_h as usize;
        let payload =
            payload.ok_or(TryFromPacketError::UnexpectedPayloadSize {
                actual: 0,
                expected,
            })?;
        let payload = match decompress(compression, &payload) {
            Ok(payload) => payload,
            Err(CompressionError::NoCompression) => payload,
            Err(_) => return Err(TryFromPacketError::DecompressionFailed),
        };
        let bitmap = Bitmap::load(
            tile_w as usize * TILE_SIZE,
            pixel_h as usize,
            &payload,
        )?;
        let origin =
            Origin::new(tiles_x as usize * TILE_SIZE, pixels_y as usize);

        Ok(Self {
            bitmap,
            origin,
            compression,
        })
    }
}

impl From<BitmapCommand> for TypedCommand {
    fn from(command: BitmapCommand) -> Self {
        Self::Bitmap(command)
    }
}

impl From<Bitmap> for BitmapCommand {
    fn from(bitmap: Bitmap) -> Self {
        Self {
            bitmap,
            origin: Origin::default(),
            compression: CompressionCode::default(),
        }
    }
}

impl BitmapCommand {
    fn command_code_for_compression(
        compression_code: CompressionCode,
    ) -> CommandCode {
        match compression_code {
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
        }
    }

    fn compression_for_command_code(
        command_code: CommandCode,
    ) -> Option<CompressionCode> {
        Some(match command_code {
            CommandCode::BitmapLinearWinUncompressed => {
                CompressionCode::Uncompressed
            }
            #[cfg(feature = "compression_zlib")]
            CommandCode::BitmapLinearWinZlib => CompressionCode::Zlib,
            #[cfg(feature = "compression_bzip2")]
            CommandCode::BitmapLinearWinBzip2 => CompressionCode::Bzip2,
            #[cfg(feature = "compression_lzma")]
            CommandCode::BitmapLinearWinLzma => CompressionCode::Lzma,
            #[cfg(feature = "compression_zstd")]
            CommandCode::BitmapLinearWinZstd => CompressionCode::Zstd,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        command_code::CommandCode, commands::tests::TestImplementsCommand,
    };

    impl TestImplementsCommand for BitmapCommand {}

    #[test]
    fn command_code() {
        assert_eq!(
            BitmapCommand::try_from(Packet {
                payload: None,
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    ..Default::default()
                }
            }),
            Err(InvalidCommandCodeError(CommandCode::Brightness.into()).into())
        );
    }

    #[test]
    fn error_decompression_failed_win() {
        for compression in CompressionCode::ALL {
            let p: Packet = BitmapCommand {
                origin: Origin::new(16, 8),
                bitmap: Bitmap::new(8, 8).unwrap(),
                compression: *compression,
            }
            .try_into()
            .unwrap();

            let Packet { header, payload } = p;
            let mut payload = payload.unwrap();

            // mangle it
            for byte in &mut payload {
                *byte -= *byte / 2;
            }

            let p = Packet {
                header,
                payload: Some(payload),
            };
            let result = TypedCommand::try_from(p);
            if *compression != CompressionCode::Uncompressed {
                assert_eq!(
                    result,
                    Err(TryFromPacketError::DecompressionFailed)
                );
            } else {
                assert!(result.is_ok());
            }
        }
    }

    #[test]
    fn into_command() {
        let mut bitmap = Bitmap::max_sized();
        bitmap.fill(true);

        assert_eq!(
            BitmapCommand::from(bitmap.clone()),
            BitmapCommand {
                bitmap,
                origin: Origin::default(),
                compression: CompressionCode::default()
            },
        )
    }

    #[test]
    fn into_packet_out_of_range() {
        let mut cmd = BitmapCommand::from(Bitmap::max_sized());
        cmd.origin.x = usize::MAX;
        assert!(matches!(
            Packet::try_from(cmd),
            Err(TryIntoPacketError::ConversionError(_))
        ));
    }

    #[test]
    fn into_packet_invalid_alignment() {
        let cmd = BitmapCommand {
            bitmap: Bitmap::max_sized(),
            compression: CompressionCode::Uncompressed,
            origin: Origin::new(5, 0),
        };
        let packet = Packet::try_from(cmd).unwrap();
        assert_eq!(
            packet.header,
            Header {
                command_code: 19,
                a: 0,
                b: 0,
                c: 56,
                d: 160
            }
        );

        let cmd = BitmapCommand {
            bitmap: Bitmap::max_sized(),
            compression: CompressionCode::Uncompressed,
            origin: Origin::new(11, 0),
        };
        let packet = Packet::try_from(cmd).unwrap();
        assert_eq!(
            packet.header,
            Header {
                command_code: 19,
                a: 1,
                b: 0,
                c: 56,
                d: 160
            }
        );
    }

    #[test]
    fn round_trip() {
        for compression in CompressionCode::ALL {
            crate::commands::tests::round_trip(
                BitmapCommand {
                    origin: Origin::ZERO,
                    bitmap: Bitmap::max_sized(),
                    compression: *compression,
                }
                .into(),
            );
        }
    }

    #[test]
    fn round_trip_ref() {
        for compression in CompressionCode::ALL {
            crate::commands::tests::round_trip_ref(
                &BitmapCommand {
                    origin: Origin::ZERO,
                    bitmap: Bitmap::max_sized(),
                    compression: *compression,
                }
                .into(),
            );
        }
    }
}
