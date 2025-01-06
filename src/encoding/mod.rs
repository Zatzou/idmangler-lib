//! Module for encoding utilities
//!
//! This module provides the main utilities for encoding and decoding data to and from the wynntils idstring private use area format.
//! This module also contains the primary encoding and decoding traits and their associated error types.

mod error;

#[doc(inline)]
pub use error::{DecodeError, DecoderError, EncodeError, EncoderError};

pub mod string;

pub(crate) mod varint;

mod traits;
#[doc(inline)]
pub use traits::{BlockId, DataDecoder, DataEncoder};
