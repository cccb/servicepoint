#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, uniffi::Enum)]
pub enum CompressionCode {
    /// no compression
    Uncompressed = 0x0,
    /// compress using flate2 with zlib header
    Zlib = 0x677a,
    /// compress using bzip2
    Bzip2 = 0x627a,
    /// compress using lzma
    Lzma = 0x6c7a,
    /// compress using Zstandard
    Zstd = 0x7a73,
}
