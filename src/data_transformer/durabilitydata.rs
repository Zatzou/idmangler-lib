use crate::{
    encoding::{decode_varint, encode_varint},
    types::TransformVersion,
};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// Durability data of a crafted item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct DurabilityData {
    /// The effect strength of the item is the overall effectiveness of the identifications on the item. (the percentage shown next to the item name)
    pub effect_strenght: u8,
    /// Current durability of the item
    pub current: i32,
    /// Maximum durability of the item
    pub max: i32,
}

impl TransformId for DurabilityData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::DurabilityData as u8;
}

impl DataEncoder for DurabilityData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                // Wynntils does not check this invariant during decoding. So lets just ignore it for fun
                // if self.effect_strenght > 100 {
                //     return Err(EncodeError::EffectStrengthTooHigh(self.effect_strenght));
                // }

                out.push(self.effect_strenght);

                out.append(&mut encode_varint(self.max as i64));

                out.append(&mut encode_varint(self.current as i64));

                Ok(())
            }
        }
    }
}

impl DataDecoder for DurabilityData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
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

impl From<DurabilityData> for AnyData {
    fn from(value: DurabilityData) -> Self {
        Self::DurabilityData(value)
    }
}
