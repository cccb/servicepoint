use crate::{CompressionCode, Payload};
use std::io::{Read, Write};

#[cfg(feature = "compression-bz")]
use bzip2::read::{BzEncoder, BzDecoder};
#[cfg(feature = "compression-gz")]
use flate2::read::{GzEncoder, GzDecoder};
#[cfg(feature = "compression-lz")]
use lz4::{EncoderBuilder as Lz4EncoderBuilder, Decoder as Lz4Decoder};
#[cfg(feature = "compression-zs")]
use zstd::{Encoder as ZstdEncoder, Decoder as ZstdDecoder};

pub(crate) fn into_decompressed(kind: CompressionCode, payload: Payload) -> Option<Payload> {
    match kind {
        CompressionCode::None => Some(payload),
        #[cfg(feature = "compression-gz")]
        CompressionCode::Gz => {
            let mut decoder = GzDecoder::new(&*payload);
            let mut decompressed = vec!();
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed)
            }
        }
        #[cfg(feature = "compression-bz")]
        CompressionCode::Bz => {
            let mut decoder = BzDecoder::new(&*payload);
            let mut decompressed = vec!();
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed)
            }
        }
        #[cfg(feature = "compression-lz")]
        CompressionCode::Lz => {
            let mut decoder = match Lz4Decoder::new(&*payload) {
                Err(_) => return None,
                Ok(value) => value
            };
            let mut decompressed = vec!();
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed)
            }
        }
        #[cfg(feature = "compression-zs")]
        CompressionCode::Zs => {
            let mut decoder = match ZstdDecoder::new(&*payload) {
                Err(_) => return None,
                Ok(value) => value
            };
            let mut decompressed = vec!();
            match decoder.read_to_end(&mut decompressed) {
                Err(_) => None,
                Ok(_) => Some(decompressed)
            }
        }
    }
}

pub(crate) fn into_compressed(kind: CompressionCode, payload: Payload) -> Payload {
    match kind {
        CompressionCode::None => payload,
        #[cfg(feature = "compression-gz")]
        CompressionCode::Gz => {
            let mut encoder = GzEncoder::new(&*payload, flate2::Compression::fast());
            let mut compressed = vec!();
            match encoder.read_to_end(&mut compressed) {
                Err(err) => panic!("could not compress payload: {}", err),
                Ok(_) => compressed,
            }
        }
        #[cfg(feature = "compression-bz")]
        CompressionCode::Bz => {
            let mut encoder = BzEncoder::new(&*payload, bzip2::Compression::fast());
            let mut compressed = vec!();
            match encoder.read_to_end(&mut compressed) {
                Err(err) => panic!("could not compress payload: {}", err),
                Ok(_) => compressed,
            }
        }
        #[cfg(feature = "compression-lz")]
        CompressionCode::Lz => {
            let mut encoder = Lz4EncoderBuilder::new()
                .build(vec!())
                .expect("could not create encoder");
            encoder.write(&*payload)
                .expect("could not write payload");
            let (payload, _) = encoder.finish();
            payload
        }
        #[cfg(feature = "compression-zs")]
        CompressionCode::Zs => {
            let mut encoder = ZstdEncoder::new(vec!(), zstd::DEFAULT_COMPRESSION_LEVEL)
                .expect("could not create encoder");
            encoder.write(&*payload)
                .expect("could not compress payload");
            encoder.finish()
                .expect("could not finish encoding")
        }
    }
}

