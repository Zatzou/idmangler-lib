//! Module containing the item types and functions for decoding and encoding items
//!
//! This module provides types for representing items encodable in the wynntils idstring format. These types have functions for decoding and encoding the items from and to strings.
//!
//! [`GenericItem`] is the main type for representing items. This type can represent any item with any data. The more specific types are used for representing specific item types.
//!
//! This module should generally be used over the block module for decoding and encoding items when low level block manipulation is not required.

use crate::{
    block::{
        AnyBlock, CraftedConsumableTypeData, CraftedGearTypeData, CraftedIdentificationData,
        DamageData, DataBlockId, DefenseData, DurabilityData, EffectsData, EndData,
        IdentificationData, NameData, PowderData, RequirementsData, RerollData, ShinyData,
        StartData, TypeData, UsesData,
    },
    encoding::{string::decode_string, EncoderError},
    types::{EncodingVersion, ItemType},
};

mod crafteds;
#[doc(inline)]
pub use crafteds::*;
pub mod error;
mod id_items;
use error::ItemDecodeError;
#[doc(inline)]
pub use id_items::*;

/// Type representing any item with any item data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl GenericItem {
    /// Decode a generic item from a list of blocks
    ///
    /// This function will attempt to decode a generic item from a list of blocks. This function will return an error if any required blocks are missing.
    pub fn from_blocks(blocks: Vec<AnyBlock>) -> Result<Self, ItemDecodeError> {
        let mut out = Self::default();
        let mut kind = None;

        for block in blocks {
            match block {
                AnyBlock::StartData(_) | AnyBlock::EndData(_) => {}

                AnyBlock::TypeData(type_data) => kind = Some(type_data.0),
                AnyBlock::NameData(name_data) => out.name = Some(name_data.0),
                AnyBlock::IdentificationData(identification_data) => {
                    out.identifications = Some(identification_data)
                }
                AnyBlock::PowderData(powder_data) => out.powders = Some(powder_data),
                AnyBlock::RerollData(reroll_data) => out.rerolls = Some(reroll_data),
                AnyBlock::ShinyData(shiny_data) => out.shiny = Some(shiny_data),
                AnyBlock::CraftedGearType(crafted_gear_type_data) => {
                    out.crafted_type = Some(crafted_gear_type_data)
                }
                AnyBlock::DurabilityData(durability_data) => {
                    out.crafted_durability = Some(durability_data)
                }
                AnyBlock::RequirementsData(requirements_data) => {
                    out.crafted_reqs = Some(requirements_data)
                }
                AnyBlock::DamageData(damage_data) => out.crafted_damage = Some(damage_data),
                AnyBlock::DefenseData(defense_data) => out.crafted_defense = Some(defense_data),
                AnyBlock::CraftedIdentificationData(crafted_identification_data) => {
                    out.crafted_identifications = Some(crafted_identification_data)
                }
                AnyBlock::CraftedConsumableTypeData(crafted_consumable_type_data) => {
                    out.crafted_consumable_type = Some(crafted_consumable_type_data)
                }
                AnyBlock::UsesData(uses_data) => out.crafted_uses = Some(uses_data),
                AnyBlock::EffectsData(effects_data) => out.crafted_effects = Some(effects_data),
            }
        }

        if let Some(kind) = kind {
            out.kind = kind;
        } else {
            return Err(ItemDecodeError::MissingBlock(DataBlockId::TypeData));
        }

        Ok(out)
    }

    /// Decode a generic item from a string
    ///
    /// This function will attempt to decode a generic item from a string. This function will return an error if any required blocks are missing.
    /// This function will also error if an error occurs while decoding the blocks.
    pub fn decode_string(input: impl AsRef<str>) -> Result<Self, ItemDecodeError> {
        let blocks = AnyBlock::decode(&mut decode_string(input)?.into_iter())?;
        Self::from_blocks(blocks)
    }

    /// Convert the generic item into a list of blocks
    ///
    /// This function will convert the generic item into a list of blocks. The list of blocks will contain all the data from the generic item.
    /// The list however will not contain any start or end blocks, which are required for a full idstring.
    pub fn into_blocks(self) -> Vec<AnyBlock> {
        [
            Some(TypeData(self.kind).into()),
            self.name.map(|n| NameData(n).into()),
            self.identifications.map(Into::into),
            self.powders.map(Into::into),
            self.rerolls.map(Into::into),
            self.shiny.map(Into::into),
            self.crafted_type.map(Into::into),
            self.crafted_durability.map(Into::into),
            self.crafted_reqs.map(Into::into),
            self.crafted_damage.map(Into::into),
            self.crafted_defense.map(Into::into),
            self.crafted_identifications.map(Into::into),
            self.crafted_consumable_type.map(Into::into),
            self.crafted_uses.map(Into::into),
            self.crafted_effects.map(Into::into),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }

    /// Encode the generic item into a string
    ///
    /// This function will encode the generic item into a string. This function will return an error if an error occurs while encoding the blocks.
    /// The string will contain all the data from the generic item and will be a valid idstring.
    pub fn encode(self, ver: EncodingVersion) -> Result<String, EncoderError> {
        let mut blocks = vec![StartData(ver).into()];

        blocks.append(&mut self.into_blocks());

        blocks.push(EndData.into());

        crate::block::encode_blocks_str(ver, &blocks)
    }
}
