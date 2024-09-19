use thiserror::Error;

use crate::DecodeError;

#[repr(i8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum AttackSpeed {
    SuperFast = 3,
    VeryFast = 2,
    Fast = 1,
    Normal = 0,
    Slow = -1,
    VerySlow = -2,
    SuperSlow = -3,
}

impl From<AttackSpeed> for u8 {
    fn from(value: AttackSpeed) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid attack speed id:`{0}`")]
pub struct BadAttackSpeed(pub i8);

impl From<BadAttackSpeed> for DecodeError {
    fn from(value: BadAttackSpeed) -> Self {
        DecodeError::BadAttackSpeed(value.0)
    }
}

impl TryFrom<u8> for AttackSpeed {
    type Error = BadAttackSpeed;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value as i8 {
            3 => Ok(Self::SuperFast),
            2 => Ok(Self::VeryFast),
            1 => Ok(Self::Fast),
            0 => Ok(Self::Normal),
            -1 => Ok(Self::Slow),
            -2 => Ok(Self::VerySlow),
            -3 => Ok(Self::SuperSlow),

            _ => Err(BadAttackSpeed(value as i8)),
        }
    }
}
