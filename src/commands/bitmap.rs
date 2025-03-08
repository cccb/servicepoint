use crate::{
    command_code::CommandCode, commands::TryFromPacketError,
    compression::into_compressed, compression::into_decompressed, Bitmap,
    CompressionCode, Grid, Header, Origin, Packet, Pixels, TypedCommand,
    TILE_SIZE,
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
/// connection.send(command).expect("send failed");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BitmapCommand {
    /// where to start drawing the pixels
    pub origin: Origin<Pixels>,
    /// the pixels to send
    pub bitmap: Bitmap,
    /// how to compress the command when converting to packet
    pub compression: CompressionCode,
}

impl From<BitmapCommand> for Packet {
    fn from(value: BitmapCommand) -> Self {
        assert_eq!(value.origin.x % 8, 0);
        assert_eq!(value.bitmap.width() % 8, 0);

        let tile_x = (value.origin.x / TILE_SIZE) as u16;
        let tile_w = (value.bitmap.width() / TILE_SIZE) as u16;
        let pixel_h = value.bitmap.height() as u16;
        let payload = into_compressed(value.compression, value.bitmap.into());
        let command = match value.compression {
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

        Packet {
            header: Header {
                command_code: command.into(),
                a: tile_x,
                b: value.origin.y as u16,
                c: tile_w,
                d: pixel_h,
            },
            payload,
        }
    }
}

impl TryFrom<Packet> for BitmapCommand {
    type Error = TryFromPacketError;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        let code = CommandCode::try_from(packet.header.command_code).map_err(
            |_| TryFromPacketError::InvalidCommand(packet.header.command_code),
        )?;

        match code {
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

            _ => Err(TryFromPacketError::InvalidCommand(
                packet.header.command_code,
            )),
        }
    }
}

impl From<BitmapCommand> for TypedCommand {
    fn from(command: BitmapCommand) -> Self {
        Self::Bitmap(command)
    }
}

impl BitmapCommand {
    fn packet_into_bitmap_win(
        packet: Packet,
        compression: CompressionCode,
    ) -> Result<Self, TryFromPacketError> {
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

        let bitmap = Bitmap::load(
            tile_w as usize * TILE_SIZE,
            pixel_h as usize,
            &payload,
        )?;

        Ok(Self {
            origin: Origin::new(
                tiles_x as usize * TILE_SIZE,
                pixels_y as usize,
            ),
            bitmap,
            compression,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_code::CommandCode;
    use crate::*;

    #[test]
    fn command_code() {
        assert_eq!(
            BitmapCommand::try_from(Packet {
                payload: vec![],
                header: Header {
                    command_code: CommandCode::Brightness.into(),
                    ..Default::default()
                }
            }),
            Err(TryFromPacketError::InvalidCommand(
                CommandCode::Brightness.into()
            ))
        );
    }

    #[test]
    fn error_decompression_failed_win() {
        for compression in CompressionCode::ALL {
            let p: Packet = commands::BitmapCommand {
                origin: Origin::new(16, 8),
                bitmap: Bitmap::new(8, 8).unwrap(),
                compression: *compression,
            }
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
            let result = TypedCommand::try_from(p);
            if *compression != CompressionCode::Uncompressed {
                assert_eq!(result, Err(TryFromPacketError::DecompressionFailed))
            } else {
                assert!(result.is_ok());
            }
        }
    }
}
