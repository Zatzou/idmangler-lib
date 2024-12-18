use thiserror::Error;

/// Enum representing the types of skillpoints
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum SkillType {
    Strength = 0,
    Dexterity = 1,
    Intelligence = 2,
    Defence = 3,
    Agility = 4,
}

impl From<SkillType> for u8 {
    fn from(value: SkillType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid skill type id:`{0}`")]
pub struct BadSkillType(pub u8);

impl TryFrom<u8> for SkillType {
    type Error = BadSkillType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Strength),
            1 => Ok(Self::Dexterity),
            2 => Ok(Self::Intelligence),
            3 => Ok(Self::Defence),
            4 => Ok(Self::Agility),

            _ => Err(BadSkillType(value)),
        }
    }
}
