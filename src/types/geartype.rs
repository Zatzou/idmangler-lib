use thiserror::Error;

/// Enum representing the possible types of gear items
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum CraftedGearType {
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

impl CraftedGearType {
    pub fn get_encode_id(&self) -> u8 {
        match self {
            CraftedGearType::Spear => 0,
            CraftedGearType::Wand => 1,
            CraftedGearType::Dagger => 2,
            CraftedGearType::Bow => 3,
            CraftedGearType::Relik => 4,

            CraftedGearType::Weapon => 12,
            CraftedGearType::Accessory => 13,

            CraftedGearType::Ring => 5,
            CraftedGearType::Bracelet => 6,
            CraftedGearType::Necklace => 7,

            CraftedGearType::Helmet => 8,
            CraftedGearType::Chestplate => 9,
            CraftedGearType::Leggings => 10,
            CraftedGearType::Boots => 11,
        }
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
