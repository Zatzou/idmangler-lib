use thiserror::Error;

use crate::encoding::DecodeError;

/// Enum for encoding the type of an item the idstring represents
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum ItemType {
    Gear = 0,
    Tome = 1,
    Charm = 2,
    CraftedGear = 3,
    CraftedConsu = 4,
}

impl From<ItemType> for u8 {
    fn from(value: ItemType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid item type id:`{0}`")]
pub struct BadItemType(pub u8);

impl From<BadItemType> for DecodeError {
    fn from(value: BadItemType) -> Self {
        DecodeError::BadItemType(value.0)
    }
}

impl TryFrom<u8> for ItemType {
    type Error = BadItemType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Gear),
            1 => Ok(Self::Tome),
            2 => Ok(Self::Charm),
            3 => Ok(Self::CraftedGear),
            4 => Ok(Self::CraftedConsu),

            _ => Err(BadItemType(value)),
        }
    }
}
