use thiserror::Error;

use crate::DecodeError;

/// Enum representing the possible types of damage
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug, Default)]
pub enum DamageType {
    Earth = 0,
    Thunder = 1,
    Fire = 2,
    Water = 3,
    Air = 4,
    #[default]
    Neutral = 5,
}

impl From<DamageType> for u8 {
    fn from(value: DamageType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid damage type id:`{0}`")]
pub struct BadDamageType(pub u8);

impl From<BadDamageType> for DecodeError {
    fn from(value: BadDamageType) -> Self {
        DecodeError::BadDamageType(value.0)
    }
}

impl TryFrom<u8> for DamageType {
    type Error = BadDamageType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Earth),
            1 => Ok(Self::Thunder),
            2 => Ok(Self::Fire),
            3 => Ok(Self::Water),
            4 => Ok(Self::Air),
            5 => Ok(Self::Neutral),

            _ => Err(BadDamageType(value)),
        }
    }
}
