/// Specifies the kind of compression to use. Availability depends on features.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::{Command, CompressionCode, Origin, PixelGrid};
/// // create command without payload compression
/// # let pixels = PixelGrid::max_sized();
/// _ = Command::BitmapLinearWin(Origin::new(0, 0), pixels, CompressionCode::Uncompressed);
///
/// // create command with payload compressed with lzma and appropriate header flags
/// # let pixels = PixelGrid::max_sized();
/// _ = Command::BitmapLinearWin(Origin::new(0, 0), pixels, CompressionCode::Lzma);
/// ```
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionCode {
    /// no compression
    Uncompressed = 0x0,
    #[cfg(feature = "compression_zlib")]
    /// compress using flate2 with zlib header
    Zlib = 0x677a,
    #[cfg(feature = "compression_bzip2")]
    /// compress using bzip2
    Bzip2 = 0x627a,
    #[cfg(feature = "compression_lzma")]
    /// compress using lzma
    Lzma = 0x6c7a,
    #[cfg(feature = "compression_zstd")]
    /// compress using Zstandard
    Zstd = 0x7a73,
}

impl From<CompressionCode> for u16 {
    fn from(value: CompressionCode) -> Self {
        value as u16
    }
}

impl TryFrom<u16> for CompressionCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            value if value == CompressionCode::Uncompressed as u16 => {
                Ok(CompressionCode::Uncompressed)
            }
            #[cfg(feature = "compression_zlib")]
            value if value == CompressionCode::Zlib as u16 => {
                Ok(CompressionCode::Zlib)
            }
            #[cfg(feature = "compression_bzip2")]
            value if value == CompressionCode::Bzip2 as u16 => {
                Ok(CompressionCode::Bzip2)
            }
            #[cfg(feature = "compression_lzma")]
            value if value == CompressionCode::Lzma as u16 => {
                Ok(CompressionCode::Lzma)
            }
            #[cfg(feature = "compression_zstd")]
            value if value == CompressionCode::Zstd as u16 => {
                Ok(CompressionCode::Zstd)
            }
            _ => Err(()),
        }
    }
}
