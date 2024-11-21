use thiserror::Error;

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
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

impl PartialOrd for AttackSpeed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AttackSpeed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Attackspeed ordering is reversed
        (u8::from(*self)).cmp(&u8::from(*other)).reverse()
    }
}
