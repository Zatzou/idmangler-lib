//! Module implementing the data blocks for the encoding of the idstrings

mod customconsutypedata;
#[doc(inline)]
pub use customconsutypedata::CustomConsumableTypeData;

mod customgeartypedata;
#[doc(inline)]
pub use customgeartypedata::CustomGearTypeData;

mod customidentdata;
#[doc(inline)]
pub use customidentdata::CustomIdentificationData;

mod damagedata;
#[doc(inline)]
pub use damagedata::DamageData;

mod defensedata;
#[doc(inline)]
pub use defensedata::DefenseData;

mod durabilitydata;
#[doc(inline)]
pub use durabilitydata::DurabilityData;

mod effectsdata;
#[doc(inline)]
pub use effectsdata::EffectsData;

mod enddata;
#[doc(inline)]
pub use enddata::EndData;

mod identdata;
#[doc(inline)]
pub use identdata::IdentificationData;

mod namedata;
#[doc(inline)]
pub use namedata::NameData;

mod powderdata;
#[doc(inline)]
pub use powderdata::PowderData;

mod requirementsdata;
#[doc(inline)]
pub use requirementsdata::RequirementsData;

mod rerolldata;
#[doc(inline)]
pub use rerolldata::RerollData;

mod shinydata;
#[doc(inline)]
pub use shinydata::ShinyData;

mod startdata;
#[doc(inline)]
pub use startdata::StartData;

mod typedata;
#[doc(inline)]
pub use typedata::TypeData;

mod usesdata;
#[doc(inline)]
pub use usesdata::UsesData;

/// Enum representing the ids of the blocks
enum DataBlockId {
    StartData = 0,
    TypeData = 1,
    NameData = 2,
    IdentificationData = 3,
    PowderData = 4,
    RerollData = 5,
    ShinyData = 6,
    CustomGearType = 7,
    DurabilityData = 8,
    RequirementsData = 9,
    DamageData = 10,
    DefenseData = 11,
    CustomIdentificationData = 12,
    CustomConsumableTypeData = 13,
    UsesData = 14,
    EffectsData = 15,
    EndData = 255,
}
