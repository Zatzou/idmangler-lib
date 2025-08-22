use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

use super::{AnyBlock, DataBlockId};

/// The block for reroll data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RerollData(pub u8);

impl BlockId for RerollData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::RerollData
    }
}

impl DataEncoder for RerollData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 => out.push(self.0),
        }

        Ok(())
    }
}

impl DataDecoder for RerollData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 => {
                Ok(Self(bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?))
            }
        }
    }
}

impl From<RerollData> for AnyBlock {
    fn from(data: RerollData) -> Self {
        AnyBlock::RerollData(data)
    }
}
