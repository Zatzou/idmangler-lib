use crate::block::{
    CraftedGearTypeData, CraftedIdentificationData, CraftedConsumableTypeData, DamageData,
    DefenseData, DurabilityData, EffectsData, EndData, IdentificationData, NameData, PowderData,
    RequirementsData, RerollData, ShinyData, StartData, TypeData, UsesData,
};

/// Represents any possible item data type
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum AnyData {
    StartData(StartData),
    TypeData(TypeData),
    NameData(NameData),
    IdentificationData(IdentificationData),
    PowderData(PowderData),
    RerollData(RerollData),
    ShinyData(ShinyData),

    CustomTypeData(CraftedGearTypeData),
    DurabilityData(DurabilityData),
    RequirementsData(RequirementsData),
    DamageData(DamageData),
    DefenseData(DefenseData),
    CustomIdentificationData(CraftedIdentificationData),
    CustomConsumableTypeData(CraftedConsumableTypeData),
    UsesData(UsesData),
    EffectsData(EffectsData),

    EndData(EndData),
}
