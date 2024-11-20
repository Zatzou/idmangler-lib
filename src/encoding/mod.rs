mod anydata;
#[doc(inline)]
pub use anydata::AnyData;

mod error;
use crate::block::{
    CraftedGearTypeData, CraftedIdentificationData, CraftedConsumableTypeData, DamageData,
    DefenseData, DurabilityData, EffectsData, EndData, IdentificationData, NameData, PowderData,
    RequirementsData, RerollData, ShinyData, StartData, TypeData, UsesData,
};
#[doc(inline)]
pub use error::{DecodeError, EncodeError};

pub mod string;

pub(crate) mod varint;

mod traits;
pub(crate) use traits::BlockId;
#[doc(inline)]
pub use traits::{DataDecoder, DataEncoder};

/// Decode the bytes contained within an idstring
pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<AnyData>, DecodeError> {
    let mut out = Vec::new();

    let mut iter = bytes.iter().copied();
    let bytes = &mut iter;

    // decode the start byte and version
    let ver = StartData::decode_start_bytes(bytes)?;

    // push the start data to the output
    out.push(AnyData::StartData(StartData(ver)));

    while let Some(id) = bytes.next() {
        match id {
            0 => return Err(DecodeError::StartReparse),
            1 => out.push(TypeData::decode_data(bytes, ver)?.into()),
            2 => out.push(NameData::decode_data(bytes, ver)?.into()),
            3 => out.push(IdentificationData::decode_data(bytes, ver)?.into()),
            4 => out.push(PowderData::decode_data(bytes, ver)?.into()),
            5 => out.push(RerollData::decode_data(bytes, ver)?.into()),
            6 => out.push(ShinyData::decode_data(bytes, ver)?.into()),

            7 => out.push(CraftedGearTypeData::decode_data(bytes, ver)?.into()),
            8 => out.push(DurabilityData::decode_data(bytes, ver)?.into()),
            9 => out.push(RequirementsData::decode_data(bytes, ver)?.into()),
            10 => out.push(DamageData::decode_data(bytes, ver)?.into()),
            11 => out.push(DefenseData::decode_data(bytes, ver)?.into()),
            12 => out.push(CraftedIdentificationData::decode_data(bytes, ver)?.into()),
            13 => out.push(CraftedConsumableTypeData::decode_data(bytes, ver)?.into()),
            14 => out.push(UsesData::decode_data(bytes, ver)?.into()),
            15 => out.push(EffectsData::decode_data(bytes, ver)?.into()),

            255 => out.push(EndData::decode_data(bytes, ver)?.into()),
            _ => return Err(DecodeError::UnknownBlock(id)),
        }
    }

    Ok(out)
}
