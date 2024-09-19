use thiserror::Error;

use crate::DecodeError;

/// Enum representing the possible types of consumables
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum ConsumableType {
    Potion = 0,
    Food = 1,
    Scroll = 2,
}

impl From<ConsumableType> for u8 {
    fn from(value: ConsumableType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid consumable type id:`{0}`")]
pub struct BadConsumableType(pub u8);

impl From<BadConsumableType> for DecodeError {
    fn from(value: BadConsumableType) -> Self {
        DecodeError::BadConsumableType(value.0)
    }
}

impl TryFrom<u8> for ConsumableType {
    type Error = BadConsumableType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Potion),
            1 => Ok(Self::Food),
            2 => Ok(Self::Scroll),

            _ => Err(BadConsumableType(value)),
        }
    }
}
