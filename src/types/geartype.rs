use thiserror::Error;

/// Enum representing the possible types of gear items
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum CraftedGearType {
    Spear = 0,
    Wand = 1,
    Dagger = 2,
    Bow = 3,
    Relik = 4,

    /// Fallback for "signed, crafted gear with a skin"
    ///
    /// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/models/gear/type/GearType.java#L27>
    Weapon = 12,
    /// Fallback for when specific gear type is not known
    Accessory = 13,

    Ring = 5,
    Bracelet = 6,
    Necklace = 7,

    Helmet = 8,
    Chestplate = 9,
    Leggings = 10,
    Boots = 11,
    // /// Cannot be encoded thru [`CustomTypeData`]! only provided for
    // MasteryTome,
    // Charm,
}

impl From<CraftedGearType> for u8 {
    fn from(value: CraftedGearType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid gear type id:`{0}` was decoded")]
pub struct BadGearType(pub u8);

impl TryFrom<u8> for CraftedGearType {
    type Error = BadGearType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CraftedGearType::Spear),
            1 => Ok(CraftedGearType::Wand),
            2 => Ok(CraftedGearType::Dagger),
            3 => Ok(CraftedGearType::Bow),
            4 => Ok(CraftedGearType::Relik),

            12 => Ok(CraftedGearType::Weapon),
            13 => Ok(CraftedGearType::Accessory),

            5 => Ok(CraftedGearType::Ring),
            6 => Ok(CraftedGearType::Bracelet),
            7 => Ok(CraftedGearType::Necklace),

            8 => Ok(CraftedGearType::Helmet),
            9 => Ok(CraftedGearType::Chestplate),
            10 => Ok(CraftedGearType::Leggings),
            11 => Ok(CraftedGearType::Boots),

            _ => Err(BadGearType(value)),
        }
    }
}
