#![deny(clippy::all)]

use brotli::CompressorWriter;
use flate2::write::GzEncoder;
use napi::bindgen_prelude::*;
use std::collections::HashMap;
use std::io::Write;

#[macro_use]
extern crate napi_derive;

#[derive(Hash, Eq, PartialEq)]
#[napi(string_enum)]
pub enum Algorithm {
  Gzip,
  BrotliCompress,
  // Deflate,
  // DeflateRaw,
}

#[derive(PartialEq)]
#[napi(string_enum)]
pub enum CompressionType {
  Fast, // Compression::fast()
  Best, // Compression::best()
}

#[napi(object)]
pub struct CompressionOptions {
  pub level: Option<u32>,
  pub compression_type: Option<CompressionType>,
}

#[napi]
pub fn get_compression_options(
  algorithm: Algorithm,
  compression_options: CompressionOptions,
) -> CompressionOptions {
  let mut default_options: HashMap<Algorithm, CompressionOptions> = HashMap::new();

  default_options.insert(
    Algorithm::Gzip,
    CompressionOptions {
      level: Some(9),
      compression_type: Some(CompressionType::Best),
    },
  );
  // default_options.insert(
  //   Algorithm::Deflate,
  //   CompressionOptions {
  //     level: Some(9),
  //     compression_type: Some(CompressionType::Best),
  //   },
  // );
  // default_options.insert(
  //   Algorithm::DeflateRaw,
  //   CompressionOptions {
  //     level: Some(9),
  //     compression_type: Some(CompressionType::Best),
  //   },
  // );
  default_options.insert(
    Algorithm::BrotliCompress,
    CompressionOptions {
      level: Some(11),
      compression_type: Some(CompressionType::Best),
    },
  );

  let default_option = default_options.get(&algorithm).unwrap();

  let final_level = match compression_options.compression_type {
    Some(CompressionType::Fast) => Some(1), // Example: Fast might correspond to level 1
    Some(CompressionType::Best) => default_option.level, // Use default level for Best
    None => compression_options.level.or(default_option.level), // Use provided or default level
  };

  CompressionOptions {
    level: final_level,
    compression_type: compression_options
      .compression_type
      .or(default_option.compression_type),
  }
}

#[napi]
pub async fn compress(
  content: Buffer,
  algorithm: Algorithm,
  options: CompressionOptions,
) -> Result<Buffer> {
  let level = options.level.unwrap_or(6);
  let compression_type = options.compression_type.unwrap_or(CompressionType::Best);

  let result = match algorithm {
    Algorithm::Gzip => {
      if compression_type == CompressionType::Fast {
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(&content)?;
        encoder.finish()?
      } else if compression_type == CompressionType::Best {
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&content)?;
        encoder.finish()?
      } else {
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::new(level));
        encoder.write_all(&content)?;
        encoder.finish()?
      }
    }
    Algorithm::BrotliCompress => {
      let mut encoder = CompressorWriter::new(Vec::new(), 4096, level, 22);
      encoder.write_all(&content)?;
      encoder.into_inner()
    }
    // Algorithm::Deflate => {
    //   let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::new(level));
    //   encoder.write_all(&content)?;
    //   encoder.finish()?
    // }
    // Algorithm::DeflateRaw => {
    //   let mut encoder = DeflateEncoder::new(Vec::new(), flate2::Compression::new(level));
    //   encoder.write_all(&content)?;
    //   encoder.finish()?
    // }
    // _ => {
    //   return Err(Error::new(
    //     Status::InvalidArg,
    //     "Invalid algorithm".to_string(),
    //   ))
    // }
  };

  Ok(result.into())
}
