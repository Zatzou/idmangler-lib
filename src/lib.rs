use encoding::{decode_bytes, string::decode_string, AnyData, DecodeError};

pub mod block;
pub mod encoding;
pub mod item;
pub mod types;

/// Decode a given idstring
pub fn decode(input: &str) -> Result<Vec<AnyData>, DecodeError> {
    let input = decode_string(input);

    decode_bytes(&input)
}
