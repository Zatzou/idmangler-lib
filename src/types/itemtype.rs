use crate::DecodeError;

/// Enum for encoding the type of an item the idstring represents
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
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

impl TryFrom<u8> for ItemType {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Gear),
            1 => Ok(Self::Tome),
            2 => Ok(Self::Charm),
            3 => Ok(Self::CraftedGear),
            4 => Ok(Self::CraftedConsu),

            _ => Err(DecodeError::InvalidUItemType(value)),
        }
    }
}
