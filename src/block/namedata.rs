use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

use super::{AnyBlock, DataBlockId};

/// The block for item name data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameData(pub String);

impl BlockId for NameData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::NameData
    }
}

impl DataEncoder for NameData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                // check that the string is valid ascii
                if !self.0.is_ascii() {
                    return Err(EncodeError::NonAsciiString);
                }

                // push the bytes
                out.extend_from_slice(self.0.as_bytes());
                // push the null terminator
                out.push(0);
            }
        }

        Ok(())
    }
}

impl DataDecoder for NameData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let b: Vec<u8> = bytes.take_while(|b| *b != 0).collect();

                // UTF-8 and ASCII share the same set of characters
                Ok(NameData(
                    String::from_utf8(b).map_err(|_| DecodeError::BadString)?,
                ))
            }
        }
    }
}

impl From<NameData> for AnyBlock {
    fn from(data: NameData) -> Self {
        AnyBlock::NameData(data)
    }
}
