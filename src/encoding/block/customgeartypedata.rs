use crate::{
    encoding::{
        traits::{DataDecoder, DataEncoder, TransformId},
        AnyData, DecodeError, EncodeError,
    },
    types::{GearType, EncodingVersion},
};

use super::DataBlockId;

/// Sets the gear type of a crafted item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CustomGearTypeData(pub GearType);

impl TransformId for CustomGearTypeData {
    const TRANSFORMER_ID: u8 = DataBlockId::CustomGearType as u8;
}

impl DataEncoder for CustomGearTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                out.push(self.0.get_encode_id());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CustomGearTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let gear = GearType::try_from(id)?;
                Ok(Self(gear))
            }
        }
    }
}

impl From<CustomGearTypeData> for AnyData {
    fn from(data: CustomGearTypeData) -> Self {
        AnyData::CustomTypeData(data)
    }
}
