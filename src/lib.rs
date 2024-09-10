mod data_transformer;
pub mod encoding;
pub mod types;

#[doc(inline)]
pub use data_transformer::{
    decode_bytes, AnyData, DataDecoder, DataEncoder, DecodeError, EncodeError, EndData,
    IdentificationData, NameData, PowderData, RerollData, ShinyData, StartData, TypeData,
};
use encoding::decode_string;

/// Decode a given idstring
pub fn decode(input: &str) -> Result<Vec<AnyData>, DecodeError> {
    let input = decode_string(input);

    decode_bytes(&input)
}
