use super::block::{
    CustomConsumableTypeData, CustomGearTypeData, CustomIdentificationData, DamageData,
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

    CustomTypeData(CustomGearTypeData),
    DurabilityData(DurabilityData),
    RequirementsData(RequirementsData),
    DamageData(DamageData),
    DefenseData(DefenseData),
    CustomIdentificationData(CustomIdentificationData),
    CustomConsumableTypeData(CustomConsumableTypeData),
    UsesData(UsesData),
    EffectsData(EffectsData),

    EndData(EndData),
}
