use block::{AnyBlock, StartData};
use encoding::string::decode_string;
use types::EncodingVersion;

pub mod block;
pub mod encoding;
pub mod item;
pub(crate) mod macros;
pub mod types;

pub fn decode(input: &str) -> Result<Vec<AnyBlock>, encoding::DecoderError> {
    AnyBlock::decode(
        &mut decode_string(input)
            .map_err(|e| encoding::DecoderError {
                error: e.into(),
                during: None,
            })?
            .into_iter(),
    )
}
