use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{Effect, EffectType, EncodingVersion},
};

use super::{anyblock::AnyBlock, DataBlockId};

/// Effects of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct EffectsData {
    pub effects: Vec<Effect>,
}

impl BlockId for EffectsData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::EffectsData
    }
}

impl DataEncoder for EffectsData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                if self.effects.len() > 255 {
                    return Err(EncodeError::TooManyEffects);
                }

                // number of effects
                out.push(self.effects.len() as u8);

                for effect in &self.effects {
                    // effect type
                    out.push(effect.kind as u8);

                    // effect value
                    out.append(&mut encode_varint(effect.value as i64));
                }

                Ok(())
            }
        }
    }
}

impl DataDecoder for EffectsData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let effect_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                let mut effects = Vec::with_capacity(effect_count as usize);

                for _ in 0..effect_count {
                    let kind = EffectType::try_from(
                        bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?,
                    )?;

                    let value = decode_varint(bytes)?;

                    effects.push(Effect {
                        kind,
                        value: value as i32,
                    });
                }

                Ok(Self { effects })
            }
        }
    }
}

impl From<EffectsData> for AnyBlock {
    fn from(data: EffectsData) -> Self {
        AnyBlock::EffectsData(data)
    }
}
