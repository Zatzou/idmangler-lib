use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{ConsumableType, EncodingVersion},
};

use super::DataBlockId;

/// Sets the type of a crafted consumable
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CraftedConsumableTypeData(pub ConsumableType);

impl BlockId for CraftedConsumableTypeData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::CustomConsumableTypeData
    }
}

impl DataEncoder for CraftedConsumableTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
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
            EncodingVersion::Version1 => {
                let kind = ConsumableType::try_from(
                    bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?,
                )?;
                Ok(Self(kind))
            }
        }
    }
}
