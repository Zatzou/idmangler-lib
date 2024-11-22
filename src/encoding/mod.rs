mod error;

#[doc(inline)]
pub use error::{DecodeError, DecoderError, EncodeError, EncoderError};

pub mod string;

pub(crate) mod varint;

mod traits;
#[doc(inline)]
pub use traits::{BlockId, DataBlock, DataDecoder, DataEncoder};
