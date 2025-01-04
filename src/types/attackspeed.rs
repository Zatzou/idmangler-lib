use crate::macros::numbered_enum;

numbered_enum! {
    #[repr(u8)]
    pub enum AttackSpeed {
        SuperFast = 0,
        VeryFast = 1,
        Fast = 2,
        Normal = 3,
        Slow = 4,
        VerySlow = 5,
        SuperSlow = 6,
    }

    #[error("Invalid attack speed id:`{0}`")]
    etype BadAttackSpeed;
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
