use crate::{
    encoding::{AnyData, BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

use super::DataBlockId;

/// The block for reroll data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct RerollData(pub u8);

impl BlockId for RerollData {
    const BLOCK_ID: u8 = DataBlockId::RerollData as u8;
}

impl DataEncoder for RerollData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => out.push(self.0),
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
            EncodingVersion::Version1 => {
                Ok(Self(bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?))
            }
        }
    }
}

impl From<RerollData> for AnyData {
    fn from(value: RerollData) -> Self {
        Self::RerollData(value)
    }
}
