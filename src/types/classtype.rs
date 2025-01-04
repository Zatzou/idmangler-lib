use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum representing the types of classes
    #[repr(u8)]
    pub enum ClassType {
        Mage = 1,
        Archer = 2,
        Warrior = 3,
        Assasin = 4,
        Shaman = 5,
    }

    #[error("Invalid class type id:`{0}`")]
    etype BadClassType;
}
