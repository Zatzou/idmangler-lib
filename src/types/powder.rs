use crate::DecodeError;

/// Powder types
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum Powders {
    EARTH = 1,
    THUNDER = 2,
    WATER = 3,
    FIRE = 4,
    AIR = 5,
}

impl TryFrom<u8> for Powders {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::EARTH),
            2 => Ok(Self::THUNDER),
            3 => Ok(Self::WATER),
            4 => Ok(Self::FIRE),
            5 => Ok(Self::AIR),
            _ => Err(DecodeError::InvalidPowder(value)),
        }
    }
}
