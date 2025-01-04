use crate::macros::numbered_enum;

numbered_enum! {
    /// Enum representing the types of skillpoints
    #[repr(u8)]
    pub enum SkillType {
        Strength = 0,
        Dexterity = 1,
        Intelligence = 2,
        Defence = 3,
        Agility = 4,
    }

    #[error("Invalid skill type id:`{0}`")]
    etype BadSkillType;
}
