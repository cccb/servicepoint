#[cfg(feature = "compression")]
use std::io::{Read, Write};

#[cfg(feature = "compression_bzip2")]
use bzip2::read::{BzDecoder, BzEncoder};
#[cfg(feature = "compression_zlib")]
use flate2::{FlushCompress, FlushDecompress, Status};
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

            let status = match decompress.decompress(
                &payload,
                &mut buffer,
                FlushDecompress::Finish,
            ) {
                Err(_) => return None,
                Ok(status) => status,
            };

            match status {
                Status::Ok => None,
                Status::BufError => None,
                Status::StreamEnd => Some(
                    buffer[0..(decompress.total_out() as usize)].to_owned(),
                ),
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
        CompressionCode::Lzma => Some(lzma::decompress(&payload).unwrap()),
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
) -> Payload {
    match kind {
        CompressionCode::Uncompressed => payload,
        #[cfg(feature = "compression_zlib")]
        CompressionCode::Zlib => {
            let mut compress =
                flate2::Compress::new(flate2::Compression::fast(), true);
            let mut buffer = [0u8; 10000];

            match compress
                .compress(&payload, &mut buffer, FlushCompress::Finish)
                .expect("compress failed")
            {
                Status::Ok => panic!("buffer should be big enough"),
                Status::BufError => panic!("BufError"),
                Status::StreamEnd => {}
            };
            buffer[..compress.total_out() as usize].to_owned()
        }
        #[cfg(feature = "compression_bzip2")]
        CompressionCode::Bzip2 => {
            let mut encoder =
                BzEncoder::new(&*payload, bzip2::Compression::fast());
            let mut compressed = vec![];
            match encoder.read_to_end(&mut compressed) {
                Err(err) => panic!("could not compress payload: {}", err),
                Ok(_) => compressed,
            }
        }
        #[cfg(feature = "compression_lzma")]
        CompressionCode::Lzma => lzma::compress(&payload, 6).unwrap(),
        #[cfg(feature = "compression_zstd")]
        CompressionCode::Zstd => {
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
