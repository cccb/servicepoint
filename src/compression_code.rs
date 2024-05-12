use CompressionCode::*;

/// Specifies the kind of compression to use. Availability depends on features.
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CompressionCode {
    Uncompressed = 0x0,
    #[cfg(feature = "compression-gz")]
    Gz = 0x677a,
    #[cfg(feature = "compression-bz")]
    Bz = 0x627a,
    #[cfg(feature = "compression-lz")]
    Lz = 0x6c7a,
    #[cfg(feature = "compression-zs")]
    Zs = 0x7a73,
}

impl Into<u16> for CompressionCode {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<u16> for CompressionCode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            value if value == Uncompressed as u16 => Ok(Uncompressed),
            #[cfg(feature = "compression-gz")]
            value if value == Gz as u16 => Ok(Gz),
            #[cfg(feature = "compression-bz")]
            value if value == Bz as u16 => Ok(Bz),
            #[cfg(feature = "compression-lz")]
            value if value == Lz as u16 => Ok(Lz),
            #[cfg(feature = "compression-zs")]
            value if value == Zs as u16 => Ok(Zs),
            _ => Err(()),
        }
    }
}
