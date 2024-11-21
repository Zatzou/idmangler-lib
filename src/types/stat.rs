/// Structure representing an identification stat as it is encoded within the wynntils format
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Stat {
    /// The id of the identification
    pub kind: u8,
    /// The base value of the identification.
    ///
    /// This value is optional when extended encoding is not used.
    ///
    /// When decoding this value may be none if the given string was encoded using non extended encoding.
    pub base: Option<i32>,
    /// The roll of this identification.
    ///
    /// The roll is either fixed or a % multiplier of the base value
    pub roll: RollType,
}

/// Enum representing the possible rolls of an [`Stat`]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum RollType {
    /// The identification has a roll between 0% and 255% of the base value.
    Value(u8),
    /// The roll of the identification is fixed meaning its final value will be the base.
    PreIdentified,
}

impl Stat {
    /// Return a boolean depending on if the identification is pre-identified or not
    pub fn pre_identified(&self) -> bool {
        match self.roll {
            RollType::Value(_) => false,
            RollType::PreIdentified => true,
        }
    }
}

/// Struct representing an identification stat on a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CraftedStat {
    /// id of the identification
    pub kind: u8,
    /// value of the identification while at full durability
    pub max: i32,
}
