use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        AnyData, BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{Effect, EffectType, EncodingVersion},
};

use super::DataBlockId;

/// Effects of a crafted item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct EffectsData {
    pub effects: Vec<Effect>,
}

impl BlockId for EffectsData {
    const BLOCK_ID: u8 = DataBlockId::EffectsData as u8;
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

impl From<EffectsData> for AnyData {
    fn from(value: EffectsData) -> Self {
        Self::EffectsData(value)
    }
}
