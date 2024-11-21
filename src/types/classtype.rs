use thiserror::Error;

/// Enum representing the types of classes
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum ClassType {
    Mage = 1,
    Archer = 2,
    Warrior = 3,
    Assasin = 4,
    Shaman = 5,
}

impl From<ClassType> for u8 {
    fn from(value: ClassType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid class type id:`{0}`")]
pub struct BadClassType(pub u8);

impl TryFrom<u8> for ClassType {
    type Error = BadClassType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClassType::Mage),
            2 => Ok(ClassType::Archer),
            3 => Ok(ClassType::Warrior),
            4 => Ok(ClassType::Assasin),
            5 => Ok(ClassType::Shaman),

            _ => Err(BadClassType(value)),
        }
    }
}
