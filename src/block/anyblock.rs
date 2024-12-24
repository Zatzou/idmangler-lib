use super::{
    CraftedConsumableTypeData, CraftedGearTypeData, CraftedIdentificationData, DamageData,
    DefenseData, DurabilityData, EffectsData, EndData, IdentificationData, NameData, PowderData,
    RequirementsData, RerollData, ShinyData, StartData, TypeData, UsesData,
};

/// Enum for representing a block of any type.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyBlock {
    StartData(StartData),
    TypeData(TypeData),
    NameData(NameData),
    IdentificationData(IdentificationData),
    PowderData(PowderData),
    RerollData(RerollData),
    ShinyData(ShinyData),
    CraftedGearType(CraftedGearTypeData),
    DurabilityData(DurabilityData),
    RequirementsData(RequirementsData),
    DamageData(DamageData),
    DefenseData(DefenseData),
    CraftedIdentificationData(CraftedIdentificationData),
    CraftedConsumableTypeData(CraftedConsumableTypeData),
    UsesData(UsesData),
    EffectsData(EffectsData),
    EndData(EndData),
}

impl AnyBlock {}
