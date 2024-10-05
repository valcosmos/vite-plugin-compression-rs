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

// #[napi(object)]
// pub struct CompressionOptions {
//   pub flush: Option<i32>,
//   pub finish_flush: Option<i32>,
//   pub chunk_size: Option<i32>,
//   pub level: Option<i32>,
//   pub mem_level: Option<i32>,
//   pub strategy: Option<i32>,
//   pub dictionary: Option<Buffer>,
//   pub info: Option<bool>,
//   pub max_output_length: Option<u32>,

//   pub params: Option<HashMap<String, i32>>,
// }

// impl ToString for Algorithm {
//   fn to_string(&self) -> String {
//     match self {
//       Algorithm::Gzip => "gzip".to_string(),
//       Algorithm::BrotliCompress => "brotliCompress".to_string(),
//       Algorithm::Deflate => "deflate".to_string(),
//       Algorithm::DeflateRaw => "deflateRaw".to_string(),
//     }
//   }
// }

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
