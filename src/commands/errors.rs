use crate::{
    command_code::InvalidCommandCodeError,
    compression_code::InvalidCompressionCodeError, LoadBitmapError,
};
use std::num::TryFromIntError;

/// Err values for [`crate::TypedCommand::try_from`].
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TryFromPacketError {
    /// the contained command code does not correspond to a known command
    #[error(transparent)]
    InvalidCommand(#[from] InvalidCommandCodeError),
    /// the expected payload size was n, but size m was found
    #[error("the expected payload size was {0}, but size {1} was found")]
    UnexpectedPayloadSize(usize, usize),
    /// Header fields not needed for the command have been used.
    ///
    /// Note that these commands would usually still work on the actual display.
    #[error("Header fields not needed for the command have been used")]
    ExtraneousHeaderValues,
    /// The contained compression code is not known. This could be of disabled features.
    #[error(transparent)]
    InvalidCompression(#[from] InvalidCompressionCodeError),
    /// Decompression of the payload failed. This can be caused by corrupted packets.
    #[error("The decompression of the payload failed")]
    DecompressionFailed,
    /// The given brightness value is out of bounds
    #[error("The given brightness value {0} is out of bounds.")]
    InvalidBrightness(u8),
    /// Some provided text was not valid UTF-8.
    #[error(transparent)]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    /// The bitmap contained in the payload could not be loaded
    #[error(transparent)]
    LoadBitmapFailed(#[from] LoadBitmapError),
}

/// An error that can occur when parsing a raw packet as a command
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TryIntoPacketError {
    /// Compression of the payload failed.
    #[error("The compression of the payload failed")]
    CompressionFailed,
    /// Conversion (probably to u16) failed
    #[error(transparent)]
    ConversionError(#[from] TryFromIntError),
}
