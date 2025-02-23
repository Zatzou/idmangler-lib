use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::EncodingVersion,
};

use super::{AnyBlock, DataBlockId};

/// The block for shiny data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShinyData {
    /// The id of the Shiny stat
    ///
    /// The ids can be found on <https://github.com/Wynntils/Static-Storage/blob/main/Data-Storage/shiny_stats.json>
    pub id: u8,
    /// The value of the given shiny stat
    pub val: i64,
}

impl BlockId for ShinyData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::ShinyData
    }
}

impl DataEncoder for ShinyData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                out.push(self.id);
                out.append(&mut encode_varint(self.val));
            }
        }

        Ok(())
    }
}

impl DataDecoder for ShinyData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let val = decode_varint(bytes)?;

                Ok(Self { id, val })
            }
        }
    }
}

impl From<ShinyData> for AnyBlock {
    fn from(data: ShinyData) -> Self {
        AnyBlock::ShinyData(data)
    }
}
