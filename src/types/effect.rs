use thiserror::Error;

/// Struct representing an effect on an item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct Effect {
    /// Type of the effect
    pub kind: EffectType,
    /// Value of the effect
    pub value: i32,
}

/// Enum representing the possible types of effects
#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum EffectType {
    Heal = 0,
    Mana = 1,
    Duration = 2,
}

impl From<EffectType> for u8 {
    fn from(value: EffectType) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid effect type: `{0}`")]
pub struct BadEffectType(u8);

impl TryFrom<u8> for EffectType {
    type Error = BadEffectType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(EffectType::Heal),
            1 => Ok(EffectType::Mana),
            2 => Ok(EffectType::Duration),
            _ => Err(BadEffectType(value)),
        }
    }
}
