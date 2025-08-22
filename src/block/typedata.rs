use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{EncodingVersion, ItemType},
};

use super::{AnyBlock, DataBlockId};

/// The block for the item type data
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeData(pub ItemType);

impl BlockId for TypeData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::TypeData
    }
}

impl DataEncoder for TypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => out.push(self.0.into()),
        }

        Ok(())
    }
}

impl DataDecoder for TypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                let b = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self(ItemType::try_from(b)?))
            }
        }
    }
}

impl From<TypeData> for AnyBlock {
    fn from(data: TypeData) -> Self {
        AnyBlock::TypeData(data)
    }
}
