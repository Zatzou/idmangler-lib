use encoding::{decode_bytes, string::decode_string, AnyData, DecodeError};

pub mod encoding;
pub mod types;

/// Decode a given idstring
pub fn decode(input: &str) -> Result<Vec<AnyData>, DecodeError> {
    let input = decode_string(input);

    decode_bytes(&input)
}
