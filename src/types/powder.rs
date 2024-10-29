use thiserror::Error;

use crate::DecodeError;

/// Powder types
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Powders {
    EARTH = 0,
    THUNDER = 1,
    WATER = 2,
    FIRE = 3,
    AIR = 4,
}

#[derive(Error, Debug)]
#[error("Invalid powder type: `{0}`")]
pub struct BadPowderType(pub u8);

impl From<BadPowderType> for DecodeError {
    fn from(value: BadPowderType) -> Self {
        DecodeError::BadPowderType(value.0)
    }
}

impl TryFrom<u8> for Powders {
    type Error = BadPowderType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EARTH),
            1 => Ok(Self::THUNDER),
            2 => Ok(Self::WATER),
            3 => Ok(Self::FIRE),
            4 => Ok(Self::AIR),
            _ => Err(BadPowderType(value)),
        }
    }
}
