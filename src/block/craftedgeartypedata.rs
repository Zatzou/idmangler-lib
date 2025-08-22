use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{CraftedGearType, EncodingVersion},
};

use super::{AnyBlock, DataBlockId};

/// Sets the gear type of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraftedGearTypeData(pub CraftedGearType);

impl BlockId for CraftedGearTypeData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::CraftedGearType
    }
}

impl DataEncoder for CraftedGearTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                out.push(self.0.into());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CraftedGearTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let gear = CraftedGearType::try_from(id)?;
                Ok(Self(gear))
            }
        }
    }
}

impl From<CraftedGearTypeData> for AnyBlock {
    fn from(data: CraftedGearTypeData) -> Self {
        AnyBlock::CraftedGearType(data)
    }
}
