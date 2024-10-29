use std::ops::Range;

use crate::{
    encoding::{decode_varint, encode_varint},
    types::{AttackSpeed, DamageType, TransformVersion},
};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// Damages of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct DamageData {
    /// Attack speed of the item
    pub attack_speed: AttackSpeed,
    /// The damage values of the item
    pub damages: Vec<(DamageType, Range<i32>)>,
}

impl TransformId for DamageData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::DamageData as u8;
}

impl DataEncoder for DamageData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                // attack speed
                out.push(self.attack_speed as u8);

                if self.damages.len() > 255 {
                    return Err(EncodeError::TooManyDamageValues);
                }

                // number of damage values
                out.push(self.damages.len() as u8);

                for (damage_type, damage_value) in self.damages.iter() {
                    // damage type
                    out.push((*damage_type).into());

                    // damage value range
                    out.append(&mut encode_varint(damage_value.start as i64));
                    out.append(&mut encode_varint(damage_value.end as i64));
                }

                Ok(())
            }
        }
    }
}

impl DataDecoder for DamageData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                // attack speed
                let attack_speed =
                    AttackSpeed::try_from(bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?)?;

                // number of damage values
                let num_damages = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let mut damages = Vec::with_capacity(num_damages as usize);

                for _ in 0..num_damages {
                    // damage type
                    let damage_type =
                        DamageType::try_from(bytes.next().ok_or(DecodeError::BadDamageType(0))?)?;

                    // damage value range
                    let start = decode_varint(bytes)? as i32;
                    let end = decode_varint(bytes)? as i32;

                    damages.push((damage_type, start..end));
                }

                Ok(Self {
                    attack_speed,
                    damages,
                })
            }
        }
    }
}

impl From<DamageData> for AnyData {
    fn from(value: DamageData) -> Self {
        Self::DamageData(value)
    }
}
