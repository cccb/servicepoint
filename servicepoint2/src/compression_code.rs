use CompressionCode::*;

/// Specifies the kind of compression to use. Availability depends on features.
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CompressionCode {
    Uncompressed = 0x0,
    #[cfg(feature = "compression_zlib")]
    Zlib = 0x677a,
    #[cfg(feature = "compression_bzip2")]
    Bzip2 = 0x627a,
    #[cfg(feature = "compression_lzma")]
    Lzma = 0x6c7a,
    #[cfg(feature = "compression_zstd")]
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
            value if value == Uncompressed as u16 => Ok(Uncompressed),
            #[cfg(feature = "compression_zlib")]
            value if value == Zlib as u16 => Ok(Zlib),
            #[cfg(feature = "compression_bzip2")]
            value if value == Bzip2 as u16 => Ok(Bzip2),
            #[cfg(feature = "compression_lzma")]
            value if value == Lzma as u16 => Ok(Lzma),
            #[cfg(feature = "compression_zstd")]
            value if value == Zstd as u16 => Ok(Zstd),
            _ => Err(()),
        }
    }
}
