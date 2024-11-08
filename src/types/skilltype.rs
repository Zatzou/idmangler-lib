use thiserror::Error;

use crate::encoding::DecodeError;

/// Enum representing the types of skillpoints
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum SkillType {
    // TODO: check that these are correct as wynntils has conflicting info about the order
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

impl From<BadSkillType> for DecodeError {
    fn from(value: BadSkillType) -> Self {
        DecodeError::BadSkillType(value.0)
    }
}

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
