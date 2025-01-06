use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

use super::{AnyBlock, DataBlockId};

/// Sets the number of uses of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct UsesData {
    /// Uses currently remaining on the item
    pub current: u8,
    /// Maximum uses of the item
    pub max: u8,
}

impl BlockId for UsesData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::UsesData
    }
}

impl DataEncoder for UsesData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                // first the current amount left
                out.push(self.current);
                // then the max amount
                out.push(self.max);

                Ok(())
            }
        }
    }
}

impl DataDecoder for UsesData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let current = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let max = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self { current, max })
            }
        }
    }
}

impl From<UsesData> for AnyBlock {
    fn from(data: UsesData) -> Self {
        AnyBlock::UsesData(data)
    }
}
