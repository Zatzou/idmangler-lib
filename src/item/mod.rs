use crate::{
    block::{
        CraftedConsumableTypeData, CraftedGearTypeData, CraftedIdentificationData, DamageData,
        DefenseData, DurabilityData, EffectsData, IdentificationData, PowderData, RequirementsData,
        RerollData, ShinyData, UsesData,
    },
    types::ItemType,
};

mod crafteds;
#[doc(inline)]
pub use crafteds::*;
pub mod error;
mod id_items;
#[doc(inline)]
pub use id_items::*;

/// Type representing any item with any item data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GenericItem {
    // generic fields
    /// The type of the item
    pub kind: ItemType,
    /// Name of the item
    pub name: Option<String>,
    /// Powders of the item
    pub powders: Option<PowderData>,

    // id item specific fields
    /// Identification data of an identifiable item
    pub identifications: Option<IdentificationData>,
    /// Reroll count of the item
    pub rerolls: Option<RerollData>,

    // shiny is specific to gear items
    /// Gear item shiny
    pub shiny: Option<ShinyData>,

    // crafted specific fields
    /// Requirements of a crafted item
    pub crafted_reqs: Option<RequirementsData>,
    /// Identifications of a crafted item
    pub crafted_identifications: Option<CraftedIdentificationData>,

    // crafted gear specific fields
    /// Crafted item type
    pub crafted_type: Option<CraftedGearTypeData>,
    /// Crafted Durability
    pub crafted_durability: Option<DurabilityData>,
    /// Crafted Damage
    pub crafted_damage: Option<DamageData>,
    /// Crafted Defense
    pub crafted_defense: Option<DefenseData>,

    // crafted consumable specific fields
    /// Crafted consumable type
    pub crafted_consumable_type: Option<CraftedConsumableTypeData>,
    /// Crafted consumable uses
    pub crafted_uses: Option<UsesData>,
    /// Crafted consumable effects
    pub crafted_effects: Option<EffectsData>,
}

impl Default for GenericItem {
    fn default() -> Self {
        Self {
            kind: ItemType::Gear,
            name: Default::default(),
            powders: Default::default(),
            identifications: Default::default(),
            rerolls: Default::default(),
            shiny: Default::default(),
            crafted_reqs: Default::default(),
            crafted_identifications: Default::default(),
            crafted_type: Default::default(),
            crafted_durability: Default::default(),
            crafted_damage: Default::default(),
            crafted_defense: Default::default(),
            crafted_consumable_type: Default::default(),
            crafted_uses: Default::default(),
            crafted_effects: Default::default(),
        }
    }
}
