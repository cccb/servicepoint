#[cfg(feature = "compression-bz")]
use bzip2::read::{BzDecoder, BzEncoder};
#[cfg(feature = "compression-gz")]
use flate2::read::{GzDecoder, GzEncoder};
#[cfg(feature = "compression-lz")]
use lz4::{Decoder as Lz4Decoder, EncoderBuilder as Lz4EncoderBuilder};
#[cfg(feature = "compression")]
use std::io::{Read, Write};
#[cfg(feature = "compression-zs")]
use zstd::{Decoder as ZstdDecoder, Encoder as ZstdEncoder};

use crate::{CompressionCode, Payload};

pub(crate) fn into_decompressed(
    kind: CompressionCode,
    payload: Payload,
) -> Option<Payload> {
    match kind {
        CompressionCode::Uncompressed => Some(payload),
        #[cfg(feature = "compression-gz")]
        CompressionCode::Gz => {
            let mut decoder = GzDecoder::new(&*payload);
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed),
            }
        }
        #[cfg(feature = "compression-bz")]
        CompressionCode::Bz => {
            let mut decoder = BzDecoder::new(&*payload);
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed),
            }
        }
        #[cfg(feature = "compression-lz")]
        CompressionCode::Lz => {
            let mut decoder = match Lz4Decoder::new(&*payload) {
                Err(_) => return None,
                Ok(value) => value,
            };
            let mut decompressed = vec![];
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed),
            }
        }
        #[cfg(feature = "compression-zs")]
        CompressionCode::Zs => {
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
) -> Payload {
    match kind {
        CompressionCode::Uncompressed => payload,
        #[cfg(feature = "compression-gz")]
        CompressionCode::Gz => {
            let mut encoder =
                GzEncoder::new(&*payload, flate2::Compression::fast());
            let mut compressed = vec![];
            match encoder.read_to_end(&mut compressed) {
                Err(err) => panic!("could not compress payload: {}", err),
                Ok(_) => compressed,
            }
        }
        #[cfg(feature = "compression-bz")]
        CompressionCode::Bz => {
            let mut encoder =
                BzEncoder::new(&*payload, bzip2::Compression::fast());
            let mut compressed = vec![];
            match encoder.read_to_end(&mut compressed) {
                Err(err) => panic!("could not compress payload: {}", err),
                Ok(_) => compressed,
            }
        }
        #[cfg(feature = "compression-lz")]
        CompressionCode::Lz => {
            let mut encoder = Lz4EncoderBuilder::new()
                .build(vec![])
                .expect("could not create encoder");
            encoder.write_all(&payload).expect("could not write payload");
            let (payload, _) = encoder.finish();
            payload
        }
        #[cfg(feature = "compression-zs")]
        CompressionCode::Zs => {
            let mut encoder =
                ZstdEncoder::new(vec![], zstd::DEFAULT_COMPRESSION_LEVEL)
                    .expect("could not create encoder");
            encoder
                .write_all(&payload)
                .expect("could not compress payload");
            encoder.finish().expect("could not finish encoding")
        }
    }
}
