pub enum Graphics {
    /// Raw is a series
    Raw,
}

/// Raw: Offset + Raw pixel content.
/// Pixels content: series of byte-sized 8 pixel
/// horizontal blocks. highest bit is the top left pixel
pub struct Raw(pub u16, pub Vec<u8>);
