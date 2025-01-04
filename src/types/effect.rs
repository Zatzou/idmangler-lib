use crate::macros::numbered_enum;

/// Struct representing an effect on an item
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Effect {
    /// Type of the effect
    pub kind: EffectType,
    /// Value of the effect
    pub value: i32,
}

numbered_enum! {
    /// Enum representing the possible types of effects
    #[repr(u8)]
    pub enum EffectType {
        Heal = 0,
        Mana = 1,
        Duration = 2,
    }

    #[error("Invalid effect type: `{0}`")]
    etype BadEffectType;
}
