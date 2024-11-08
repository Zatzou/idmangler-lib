use thiserror::Error;

use crate::encoding::DecodeError;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum AttackSpeed {
    SuperFast = 0,
    VeryFast = 1,
    Fast = 2,
    Normal = 3,
    Slow = 4,
    VerySlow = 5,
    SuperSlow = 6,
}

impl From<AttackSpeed> for u8 {
    fn from(value: AttackSpeed) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid attack speed id:`{0}`")]
pub struct BadAttackSpeed(pub u8);

impl From<BadAttackSpeed> for DecodeError {
    fn from(value: BadAttackSpeed) -> Self {
        DecodeError::BadAttackSpeed(value.0)
    }
}

impl TryFrom<u8> for AttackSpeed {
    type Error = BadAttackSpeed;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SuperFast),
            1 => Ok(Self::VeryFast),
            2 => Ok(Self::Fast),
            3 => Ok(Self::Normal),
            4 => Ok(Self::Slow),
            5 => Ok(Self::VerySlow),
            6 => Ok(Self::SuperSlow),

            _ => Err(BadAttackSpeed(value)),
        }
    }
}
