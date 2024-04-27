//! # PNG encoder and decoder
//! This crate contains a PNG decoder. It supports reading of single lines or whole frames.
//! ## The decoder
//! The most important types for decoding purposes are [`Decoder`](struct.Decoder.html) and
//! [`Reader`](struct.Reader.html). They both wrap a `std::io::Read`.
//! `Decoder` serves as a builder for `Reader`. Calling `Decoder::read_info` reads from the `Read` until the
//! image data is reached.
//! ### Using the decoder
//!     use std::fs::File;
//!
//!     // The decoder is a build for reader and can be used to set various decoding options
//!     // via `Transformations`. The default output transformation is `TRANSFORM_EXPAND
//!     // | TRANSFORM_STRIP_ALPHA`.
//!     let decoder = png::Decoder::new(File::open("tests/pngsuite/basi0g01.png").unwrap());
//!     let (info, mut reader) = decoder.read_info().unwrap();
//!     // Allocate the output buffer.
//!     let mut buf = vec![0; info.buffer_size()];
//!     // Read the next frame. Currently this function should only called once.
//!     // The default options
//!     reader.next_frame(&mut buf).unwrap();
//! ## Encoder
//! Not available yet
//#![cfg_attr(test, feature(test))]

#[macro_use] extern crate bitflags;

extern crate num_iter;

pub mod chunk;
mod crc;
mod decoder;
#[cfg(feature = "png-encoding")]
mod encoder;
mod filter;
mod traits;
mod common;
mod utils;

pub use common::*;
pub use decoder::{Decoder, Reader, OutputInfo, StreamingDecoder, Decoded, DecodingError};
#[cfg(feature = "png-encoding")]
pub use encoder::{Encoder, Writer, EncodingError};

pub use traits::{Parameter, HasParameters};
