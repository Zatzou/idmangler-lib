use crate::{
    encoding::{
        traits::{BlockId, DataDecoder, DataEncoder},
        AnyData, DecodeError, EncodeError,
    },
    types::{EncodingVersion, ItemType},
};

use super::DataBlockId;

/// The block for the item type data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct TypeData(pub ItemType);

impl BlockId for TypeData {
    const BLOCK_ID: u8 = DataBlockId::TypeData as u8;
}

impl DataEncoder for TypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => out.push(self.0.into()),
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
            EncodingVersion::Version1 => {
                let b = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self(ItemType::try_from(b)?))
            }
        }
    }
}

impl From<TypeData> for AnyData {
    fn from(value: TypeData) -> Self {
        Self::TypeData(value)
    }
}
