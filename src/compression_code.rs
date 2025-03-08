/// Specifies the kind of compression to use. Availability depends on features.
///
/// # Examples
///
/// ```rust
/// # use servicepoint::*;
/// // create command without payload compression
/// # let pixels = Bitmap::max_sized();
/// _ = BitmapCommand {
///     origin: Origin::ZERO,
///     bitmap: pixels,
///     compression: CompressionCode::Uncompressed
/// };
///
/// // create command with payload compressed with lzma and appropriate header flags
/// # let pixels = Bitmap::max_sized();
/// _ = BitmapCommand {
///     origin: Origin::ZERO,
///     bitmap: pixels,
///     compression: CompressionCode::Lzma
/// };
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

impl CompressionCode {
    /// All available compression codes (depending on features).
    pub const ALL: &'static [CompressionCode] = &[
        Self::Uncompressed,
        #[cfg(feature = "compression_zlib")]
        Self::Zlib,
        #[cfg(feature = "compression_bzip2")]
        Self::Bzip2,
        #[cfg(feature = "compression_lzma")]
        Self::Lzma,
        #[cfg(feature = "compression_zstd")]
        Self::Zstd,
    ];
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

impl Default for CompressionCode {
    #[cfg(feature = "compression_lzma")]
    fn default() -> Self {
        CompressionCode::Lzma
    }
    #[cfg(not(feature = "compression_lzma"))]
    fn default() -> Self {
        CompressionCode::Uncompressed
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn uncompressed() {
        assert_eq!(
            CompressionCode::try_from(0x0000),
            Ok(CompressionCode::Uncompressed)
        );
        assert_eq!(u16::from(CompressionCode::Uncompressed), 0x0000);
    }

    #[test]
    #[cfg(feature = "compression_zlib")]
    fn zlib() {
        assert_eq!(
            CompressionCode::try_from(0x677a),
            Ok(CompressionCode::Zlib)
        );
        assert_eq!(u16::from(CompressionCode::Zlib), 0x677a);
    }

    #[test]
    #[cfg(feature = "compression_bzip2")]
    fn bzip2() {
        assert_eq!(
            CompressionCode::try_from(0x627a),
            Ok(CompressionCode::Bzip2)
        );
        assert_eq!(u16::from(CompressionCode::Bzip2), 0x627a);
    }

    #[test]
    #[cfg(feature = "compression_lzma")]
    fn lzma() {
        assert_eq!(
            CompressionCode::try_from(0x6c7a),
            Ok(CompressionCode::Lzma)
        );
        assert_eq!(u16::from(CompressionCode::Lzma), 0x6c7a);
    }

    #[test]
    #[cfg(feature = "compression_zstd")]
    fn zstd() {
        assert_eq!(
            CompressionCode::try_from(0x7a73),
            Ok(CompressionCode::Zstd)
        );
        assert_eq!(u16::from(CompressionCode::Zstd), 0x7a73);
    }

    #[test]
    #[cfg(feature = "compression_lzma")]
    fn default_lzma() {
        assert_eq!(CompressionCode::default(), CompressionCode::Lzma);
    }

    #[test]
    #[cfg(not(feature = "compression_lzma"))]
    fn default_uncompressed() {
        assert_eq!(CompressionCode::default(), CompressionCode::Uncompressed);
    }
}
