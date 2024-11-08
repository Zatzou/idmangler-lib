use thiserror::Error;

use crate::encoding::DecodeError;

/// Enum representing the possible types of gear items
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum GearType {
    Spear,
    Wand,
    Dagger,
    Bow,
    Relik,

    /// Fallback for "signed, crafted gear with a skin"
    ///
    /// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/models/gear/type/GearType.java#L27>
    Weapon,
    /// Fallback for when specific gear type is not known
    Accessory,

    Ring,
    Bracelet,
    Necklace,

    Helmet,
    Chestplate,
    Leggings,
    Boots,
    // /// Cannot be encoded thru [`CustomTypeData`]! only provided for
    // MasteryTome,
    // Charm,
}

impl GearType {
    pub fn get_encode_id(&self) -> u8 {
        match self {
            GearType::Spear => 0,
            GearType::Wand => 1,
            GearType::Dagger => 2,
            GearType::Bow => 3,
            GearType::Relik => 4,

            GearType::Weapon => 12,
            GearType::Accessory => 13,

            GearType::Ring => 5,
            GearType::Bracelet => 6,
            GearType::Necklace => 7,

            GearType::Helmet => 8,
            GearType::Chestplate => 9,
            GearType::Leggings => 10,
            GearType::Boots => 11,
        }
    }
}

#[derive(Error, Debug)]
#[error("Invalid gear type id:`{0}` was decoded")]
pub struct BadGearType(pub u8);

impl From<BadGearType> for DecodeError {
    fn from(value: BadGearType) -> Self {
        DecodeError::BadGearType(value.0)
    }
}

impl TryFrom<u8> for GearType {
    type Error = BadGearType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GearType::Spear),
            1 => Ok(GearType::Wand),
            2 => Ok(GearType::Dagger),
            3 => Ok(GearType::Bow),
            4 => Ok(GearType::Relik),

            12 => Ok(GearType::Weapon),
            13 => Ok(GearType::Accessory),

            5 => Ok(GearType::Ring),
            6 => Ok(GearType::Bracelet),
            7 => Ok(GearType::Necklace),

            8 => Ok(GearType::Helmet),
            9 => Ok(GearType::Chestplate),
            10 => Ok(GearType::Leggings),
            11 => Ok(GearType::Boots),

            _ => Err(BadGearType(value)),
        }
    }
}
