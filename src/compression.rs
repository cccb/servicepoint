#[cfg(feature = "compression_bzip2")]
use bzip2::read::{BzDecoder, BzEncoder};
#[cfg(feature = "compression_zlib")]
use flate2::{FlushCompress, FlushDecompress, Status};
#[allow(unused, reason = "used depending on enabled features")]
use log::error;
#[allow(unused, reason = "used depending on enabled features")]
use std::io::{Read, Write};
#[cfg(feature = "compression_zstd")]
use zstd::{Decoder as ZstdDecoder, Encoder as ZstdEncoder};

use crate::{CompressionCode, Payload};

#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum CompressionError {
    #[error("Could not compress or decompress as no compression is used.")]
    NoCompression,
    #[error("Could not initialize compression library")]
    #[allow(unused, reason = "depends on features")]
    LibraryError,
    #[error("Compression/decompression operation failed")]
    #[allow(unused, reason = "depends on features")]
    CompressionFailed,
}

pub(crate) fn decompress(
    kind: CompressionCode,
    #[allow(unused, reason = "depends on features")]
    payload: &[u8],
) -> Result<Payload, CompressionError> {
    match kind {
        CompressionCode::Uncompressed => Err(CompressionError::NoCompression),
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => {
            let mut decompress = flate2::Decompress::new(true);
            let mut buffer = [0u8; 10000];

            match decompress.decompress(
                payload,
                &mut buffer,
                FlushDecompress::Finish,
            ) {
                Ok(Status::Ok) => {
                    error!("input not big enough");
                    Err(CompressionError::CompressionFailed)
                }
                Ok(Status::BufError) => {
                    error!("output buffer is too small");
                    Err(CompressionError::CompressionFailed)
                }
                Ok(Status::StreamEnd) =>
                {
                    #[allow(
                        clippy::cast_possible_truncation,
                        reason = "can never be larger than the fixed buffer size"
                    )]
                    Ok(buffer[..decompress.total_out() as usize].to_owned())
                }
                Err(e) => {
                    error!("failed to decompress data: {e}");
                    Err(CompressionError::CompressionFailed)
                }
            }
        }
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => {
            let mut decoder = BzDecoder::new(payload);
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Ok(_) => Ok(decompressed),
                Err(e) => {
                    error!("failed to decompress data: {e}");
                    Err(CompressionError::CompressionFailed)
                }
            }
        }
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => lzma::decompress(payload).map_err(|e| {
            error!("failed to decompress data: {e}");
            CompressionError::CompressionFailed
        }),
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => {
            let mut decoder = match ZstdDecoder::new(payload) {
                Ok(value) => value,
                Err(e) => {
                    error!("failed to create zstd decoder: {e}");
                    return Err(CompressionError::LibraryError);
                }
            };
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(e) => {
                    error!("failed to decompress data: {e}");
                    Err(CompressionError::CompressionFailed)
                }
                Ok(_) => Ok(decompressed),
            }
        }
    }
}

pub(crate) fn compress(
    kind: CompressionCode,
    #[allow(unused, reason = "depends on features")]
    payload: &[u8],
) -> Result<Payload, CompressionError> {
    match kind {
        CompressionCode::Uncompressed => Err(CompressionError::NoCompression),
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => {
            let mut compress =
                flate2::Compress::new(flate2::Compression::fast(), true);
            let mut buffer = [0u8; 10000];

            match compress.compress(
                payload,
                &mut buffer,
                FlushCompress::Finish,
            ) {
                Ok(Status::Ok) => {
                    error!("output buffer not big enough");
                    Err(CompressionError::CompressionFailed)
                }
                Ok(Status::BufError) => {
                    error!("Could not compress with buffer error");
                    Err(CompressionError::CompressionFailed)
                }
                Ok(Status::StreamEnd) =>
                {
                    #[allow(
                        clippy::cast_possible_truncation,
                        reason = "can never be larger than the fixed buffer size"
                    )]
                    Ok(buffer[..compress.total_out() as usize].to_owned())
                }
                Err(e) => {
                    error!("failed to compress data: {e}");
                    Err(CompressionError::CompressionFailed)
                }
            }
        }
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => {
            let mut encoder =
                BzEncoder::new(payload, bzip2::Compression::fast());
            let mut compressed = vec![];
            match encoder.read_to_end(&mut compressed) {
                Ok(_) => Ok(compressed),
                Err(e) => {
                    error!("failed to compress data: {e}");
                    Err(CompressionError::CompressionFailed)
                }
            }
        }
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => lzma::compress(payload, 6).map_err(|e| {
            error!("failed to compress data: {e}");
            CompressionError::CompressionFailed
        }),
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => {
            let buf = Vec::with_capacity(payload.len());
            let mut encoder =
                ZstdEncoder::new(buf, zstd::DEFAULT_COMPRESSION_LEVEL)
                    .map_err(|e| {
                        error!("failed to create zstd encoder: {e}");
                        CompressionError::LibraryError
                    })?;

            if let Err(e) = encoder.write_all(payload) {
                error!("failed to compress data: {e}");
                return Err(CompressionError::CompressionFailed);
            }

            encoder.finish().map_err(|e| {
                error!("failed to finish compression: {e}");
                CompressionError::CompressionFailed
            })
        }
    }
}
