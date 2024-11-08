use crate::{
    encoding::{
        traits::{BlockId, DataDecoder, DataEncoder},
        AnyData, DecodeError, EncodeError,
    },
    types::{ConsumableType, EncodingVersion},
};

use super::DataBlockId;

/// Sets the type of a crafted consumable
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CustomConsumableTypeData(pub ConsumableType);

impl BlockId for CustomConsumableTypeData {
    const BLOCK_ID: u8 = DataBlockId::CustomConsumableTypeData as u8;
}

impl DataEncoder for CustomConsumableTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                out.push(self.0.into());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CustomConsumableTypeData {
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

impl From<CustomConsumableTypeData> for AnyData {
    fn from(value: CustomConsumableTypeData) -> Self {
        Self::CustomConsumableTypeData(value)
    }
}
