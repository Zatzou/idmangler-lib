use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum representing the elements
    #[repr(u8)]
    pub enum Element {
        Earth = 0,
        Thunder = 1,
        Water = 2,
        Fire = 3,
        Air = 4,
    }

    #[error("Invalid element id:`{0}`")]
    etype BadElement;
}
