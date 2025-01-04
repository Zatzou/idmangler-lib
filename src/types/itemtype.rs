use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum for encoding the type of an item the idstring represents
    #[repr(u8)]
    pub enum ItemType {
        Gear = 0,
        Tome = 1,
        Charm = 2,
        CraftedGear = 3,
        CraftedConsu = 4,
    }

    #[error("Invalid item type id:`{0}`")]
    etype BadItemType;
}
