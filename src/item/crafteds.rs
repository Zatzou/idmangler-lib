use crate::{
    block::{
        AnyBlock, CraftedConsumableTypeData, CraftedGearTypeData, CraftedIdentificationData,
        DamageData, DefenseData, DurabilityData, EffectsData, PowderData, RequirementsData,
        UsesData,
    },
    encoding::EncoderError,
    types::{EncodingVersion, ItemType},
};

use super::{
    error::{ItemConvertError, ItemDecodeError},
    GenericItem,
};

/// Crafted gear item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraftedGear {
    /// The type of the crafted item
    gear_type: CraftedGearTypeData,
    /// Durability of the crafted item
    durability: DurabilityData,
    /// Requirements of the crafted item
    requirements: RequirementsData,

    // other fields
    /// Name of the crafted item
    name: Option<String>,
    /// Damage of the crafted item
    damage: Option<DamageData>,
    /// Defense of the crafted item
    defense: Option<DefenseData>,
    /// Identifications of the crafted item
    identifications: Option<CraftedIdentificationData>,
    /// Powders of the crafted item
    powders: Option<PowderData>,
}

impl TryFrom<GenericItem> for CraftedGear {
    type Error = ItemConvertError;

    fn try_from(value: GenericItem) -> Result<Self, Self::Error> {
        if value.kind == ItemType::CraftedGear {
            Ok(Self {
                gear_type: value
                    .crafted_type
                    .ok_or_else(|| ItemConvertError::MissingField("crafted_type".to_string()))?,
                durability: value.crafted_durability.ok_or_else(|| {
                    ItemConvertError::MissingField("crafted_durability".to_string())
                })?,
                requirements: value
                    .crafted_reqs
                    .ok_or_else(|| ItemConvertError::MissingField("crafted_reqs".to_string()))?,
                name: value.name,
                damage: value.crafted_damage,
                defense: value.crafted_defense,
                identifications: value.crafted_identifications,
                powders: value.powders,
            })
        } else {
            Err(ItemConvertError::InvalidItemType(value.kind))
        }
    }
}

impl From<CraftedGear> for GenericItem {
    fn from(value: CraftedGear) -> Self {
        Self {
            kind: ItemType::CraftedGear,
            name: value.name,
            crafted_type: Some(value.gear_type),
            crafted_durability: Some(value.durability),
            crafted_reqs: Some(value.requirements),
            crafted_damage: value.damage,
            crafted_defense: value.defense,
            crafted_identifications: value.identifications,
            powders: value.powders,
            ..Default::default()
        }
    }
}

impl CraftedGear {
    /// See [`GenericItem::from_blocks`]
    pub fn from_blocks(blocks: Vec<AnyBlock>) -> Result<Self, ItemDecodeError> {
        let generic = GenericItem::from_blocks(blocks)?;

        Ok(Self::try_from(generic)?)
    }

    /// See [`GenericItem::decode_string`]
    pub fn decode_string(input: &str) -> Result<Self, ItemDecodeError> {
        let generic = GenericItem::decode_string(input)?;

        Ok(Self::try_from(generic)?)
    }

    /// See [`GenericItem::into_blocks`]
    pub fn into_blocks(self) -> Vec<AnyBlock> {
        GenericItem::from(self).into_blocks()
    }

    /// See [`GenericItem::encode`]
    pub fn encode(self, ver: EncodingVersion) -> Result<String, EncoderError> {
        GenericItem::from(self).encode(ver)
    }
}

/// Crafted consumable
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraftedConsumable {
    /// Consumable type
    pub consumable_type: CraftedConsumableTypeData,
    /// Uses
    pub uses: UsesData,
    /// Requirements
    pub requirements: RequirementsData,

    // other fields
    /// Name of the crafted item
    name: Option<String>,
    /// Effects of the crafted item
    effects: Option<EffectsData>,
    /// Identifications of the crafted item
    identifications: Option<CraftedIdentificationData>,
}

impl TryFrom<GenericItem> for CraftedConsumable {
    type Error = ItemConvertError;

    fn try_from(value: GenericItem) -> Result<Self, Self::Error> {
        if value.kind == ItemType::CraftedConsu {
            Ok(Self {
                consumable_type: value.crafted_consumable_type.ok_or_else(|| {
                    ItemConvertError::MissingField("crafted_consumable_type".to_string())
                })?,
                uses: value
                    .crafted_uses
                    .ok_or_else(|| ItemConvertError::MissingField("crafted_uses".to_string()))?,
                requirements: value
                    .crafted_reqs
                    .ok_or_else(|| ItemConvertError::MissingField("crafted_reqs".to_string()))?,
                name: value.name,
                effects: value.crafted_effects,
                identifications: value.crafted_identifications,
            })
        } else {
            Err(ItemConvertError::InvalidItemType(value.kind))
        }
    }
}

impl From<CraftedConsumable> for GenericItem {
    fn from(value: CraftedConsumable) -> Self {
        Self {
            kind: ItemType::CraftedConsu,
            name: value.name,
            crafted_consumable_type: Some(value.consumable_type),
            crafted_uses: Some(value.uses),
            crafted_reqs: Some(value.requirements),
            crafted_effects: value.effects,
            crafted_identifications: value.identifications,
            ..Default::default()
        }
    }
}

impl CraftedConsumable {
    /// See [`GenericItem::from_blocks`]
    pub fn from_blocks(blocks: Vec<AnyBlock>) -> Result<Self, ItemDecodeError> {
        let generic = GenericItem::from_blocks(blocks)?;

        Ok(Self::try_from(generic)?)
    }

    /// See [`GenericItem::decode_string`]
    pub fn decode_string(input: &str) -> Result<Self, ItemDecodeError> {
        let generic = GenericItem::decode_string(input)?;

        Ok(Self::try_from(generic)?)
    }

    /// See [`GenericItem::into_blocks`]
    pub fn into_blocks(self) -> Vec<AnyBlock> {
        GenericItem::from(self).into_blocks()
    }

    /// See [`GenericItem::encode`]
    pub fn encode(self, ver: EncodingVersion) -> Result<String, EncoderError> {
        GenericItem::from(self).encode(ver)
    }
}
