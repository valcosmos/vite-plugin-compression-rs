#![deny(clippy::all)]

use brotli::CompressorWriter;
use flate2::write::{DeflateEncoder, GzEncoder, ZlibEncoder};
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
  Deflate,
  DeflateRaw,
}


#[napi(object)]
pub struct CompressionOptions {
  pub level: Option<u32>,
}

#[napi]
pub fn get_compression_options(
  algorithm: Algorithm,
  compression_options: CompressionOptions,
) -> CompressionOptions {
  let mut default_options: HashMap<Algorithm, CompressionOptions> = HashMap::new();

  default_options.insert(Algorithm::Gzip, CompressionOptions { level: Some(9) });
  default_options.insert(Algorithm::Deflate, CompressionOptions { level: Some(9) });
  default_options.insert(Algorithm::DeflateRaw, CompressionOptions { level: Some(9) });
  default_options.insert(
    Algorithm::BrotliCompress,
    CompressionOptions { level: Some(11) },
  );

  let default_option = default_options.get(&algorithm).unwrap();

  CompressionOptions {
    level: compression_options.level.or(default_option.level),
  }
}

#[napi]
pub async fn compress(
  content: Buffer,
  algorithm: Algorithm,
  options: CompressionOptions,
) -> Result<Buffer> {
  let level = options.level.unwrap_or(6);

  let result = match algorithm {
    Algorithm::Gzip => {
      let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::new(level));
      encoder.write_all(&content)?;
      encoder.finish()?
    }
    Algorithm::BrotliCompress => {
      let mut encoder = CompressorWriter::new(Vec::new(), 4096, level, 22);
      encoder.write_all(&content)?;
      encoder.into_inner()
    }
    Algorithm::Deflate => {
      let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::new(level));
      encoder.write_all(&content)?;
      encoder.finish()?
    }
    Algorithm::DeflateRaw => {
      let mut encoder = DeflateEncoder::new(Vec::new(), flate2::Compression::new(level));
      encoder.write_all(&content)?;
      encoder.finish()?
    }
    _ => {
      return Err(Error::new(
        Status::InvalidArg,
        "Invalid algorithm".to_string(),
      ))
    }
  };

  Ok(result.into())
}
