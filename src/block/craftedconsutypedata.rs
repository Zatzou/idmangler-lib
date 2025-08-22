use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{ConsumableType, EncodingVersion},
};

use super::{AnyBlock, DataBlockId};

/// Sets the type of a crafted consumable
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraftedConsumableTypeData(pub ConsumableType);

impl BlockId for CraftedConsumableTypeData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::CraftedConsumableTypeData
    }
}

impl DataEncoder for CraftedConsumableTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                out.push(self.0.into());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CraftedConsumableTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                let kind = ConsumableType::try_from(
                    bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?,
                )?;
                Ok(Self(kind))
            }
        }
    }
}

impl From<CraftedConsumableTypeData> for AnyBlock {
    fn from(data: CraftedConsumableTypeData) -> Self {
        AnyBlock::CraftedConsumableTypeData(data)
    }
}
