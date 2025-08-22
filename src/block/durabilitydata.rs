use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::EncodingVersion,
};

use super::{AnyBlock, DataBlockId};

/// Durability data of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DurabilityData {
    /// The effect strength of the item is the overall effectiveness of the identifications on the item. (the percentage shown next to the item name)
    // TODO: fix misspelling in the next minor version
    pub effect_strenght: u8,
    /// Current durability of the item
    pub current: i32,
    /// Maximum durability of the item
    pub max: i32,
}

impl BlockId for DurabilityData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::DurabilityData
    }
}

impl DataEncoder for DurabilityData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 => {
                // Wynntils does not check this invariant during decoding. So lets just ignore it for fun
                // if self.effect_strenght > 100 {
                //     return Err(EncodeError::EffectStrengthTooHigh(self.effect_strenght));
                // }

                out.push(self.effect_strenght);

                out.append(&mut encode_varint(self.max));

                out.append(&mut encode_varint(self.current));

                Ok(())
            }
        }
    }
}

impl DataDecoder for DurabilityData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 => {
                let effect_strenght = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                let max = decode_varint(bytes)? as i32;

                let current = decode_varint(bytes)? as i32;

                Ok(Self {
                    effect_strenght,
                    current,
                    max,
                })
            }
        }
    }
}

impl From<DurabilityData> for AnyBlock {
    fn from(data: DurabilityData) -> Self {
        AnyBlock::DurabilityData(data)
    }
}
