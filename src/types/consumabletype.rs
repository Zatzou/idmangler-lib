use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum representing the possible types of consumables
    #[repr(u8)]
    pub enum ConsumableType {
        Potion = 0,
        Food = 1,
        Scroll = 2,
    }

    #[error("Invalid consumable type id:`{0}`")]
    etype BadConsumableType;
}
