#[cfg(feature = "compression_bzip2")]
use bzip2::read::{BzDecoder, BzEncoder};
#[cfg(feature = "compression_zlib")]
use flate2::{FlushCompress, FlushDecompress, Status};
use log::error;
#[allow(unused)]
use std::io::{Read, Write};
#[cfg(feature = "compression_zstd")]
use zstd::{Decoder as ZstdDecoder, Encoder as ZstdEncoder};

use crate::{CompressionCode, Payload};

pub(crate) fn into_decompressed(
    kind: CompressionCode,
    payload: Payload,
) -> Option<Payload> {
    match kind {
        CompressionCode::Uncompressed => Some(payload),
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => {
            let mut decompress = flate2::Decompress::new(true);
            let mut buffer = [0u8; 10000];

            match decompress.decompress(
                &payload,
                &mut buffer,
                FlushDecompress::Finish,
            ) {
                Ok(Status::Ok) => {
                    error!("input not big enough");
                    None
                }
                Ok(Status::BufError) => {
                    error!("output buffer is too small");
                    None
                }
                Ok(Status::StreamEnd) =>
                {
                    #[allow(
                        clippy::cast_possible_truncation,
                        reason = "can never be larger than the fixed buffer size"
                    )]
                    Some(buffer[..decompress.total_out() as usize].to_owned())
                }
                Err(e) => {
                    error!("compress returned err: {e}");
                    None
                }
            }
        }
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => {
            let mut decoder = BzDecoder::new(&*payload);
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed),
            }
        }
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => match lzma::decompress(&payload) {
            Err(_) => None,
            Ok(decompressed) => Some(decompressed),
        },
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => {
            let mut decoder = match ZstdDecoder::new(&*payload) {
                Err(_) => return None,
                Ok(value) => value,
            };
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed),
            }
        }
    }
}

pub(crate) fn into_compressed(
    kind: CompressionCode,
    payload: Payload,
) -> Option<Payload> {
    match kind {
        CompressionCode::Uncompressed => Some(payload),
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => {
            let mut compress =
                flate2::Compress::new(flate2::Compression::fast(), true);
            let mut buffer = [0u8; 10000];

            match compress.compress(
                &payload,
                &mut buffer,
                FlushCompress::Finish,
            ) {
                Ok(Status::Ok) => {
                    error!("output buffer not big enough");
                    None
                }
                Ok(Status::BufError) => {
                    error!("Could not compress with buffer error");
                    None
                }
                Ok(Status::StreamEnd) =>
                {
                    #[allow(
                        clippy::cast_possible_truncation,
                        reason = "can never be larger than the fixed buffer size"
                    )]
                    Some(buffer[..compress.total_out() as usize].to_owned())
                }
                Err(e) => {
                    error!("compress returned err: {e}");
                    None
                }
            }
        }
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => {
            let mut encoder =
                BzEncoder::new(&*payload, bzip2::Compression::fast());
            let mut compressed = vec![];
            match encoder.read_to_end(&mut compressed) {
                Err(err) => {
                    error!("Could not compress: {:?}", err);
                    None
                }
                Ok(_) => Some(compressed),
            }
        }
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => match lzma::compress(&payload, 6) {
            Ok(payload) => Some(payload),
            Err(e) => {
                error!("Could not compress: {e:?}");
                None
            }
        },
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => {
            let buf = Vec::with_capacity(payload.len());
            let mut encoder =
                match ZstdEncoder::new(buf, zstd::DEFAULT_COMPRESSION_LEVEL) {
                    Err(e) => {
                        error!("failed to create decoder: {e:?}");
                        return None;
                    }
                    Ok(encoder) => encoder,
                };

            if let Err(e) = encoder.write_all(&payload) {
                error!("failed to decompress payload: {e:?}");
                return None;
            }

            encoder.finish().ok()
        }
    }
}
