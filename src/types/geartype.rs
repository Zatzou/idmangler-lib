use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum representing the possible types of gear items
    #[repr(u8)]
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

    #[error("Invalid gear type id:`{0}`")]
    etype BadGearType;
}
