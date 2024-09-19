mod data_transformer;
pub mod encoding;
pub mod types;

#[doc(inline)]
pub use data_transformer::{
    decode_bytes, AnyData, CustomConsumableTypeData, CustomIdentificationData, CustomTypeData,
    DamageData, DataDecoder, DataEncoder, DecodeError, DefenseData, DurabilityData, EffectsData,
    EncodeError, EndData, IdentificationData, NameData, PowderData, RequirementsData, RerollData,
    ShinyData, StartData, TypeData, UsesData,
};
use encoding::decode_string;

/// Decode a given idstring
pub fn decode(input: &str) -> Result<Vec<AnyData>, DecodeError> {
    let input = decode_string(input);

    decode_bytes(&input)
}
