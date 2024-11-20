use crate::{
    block::{IdentificationData, PowderData, RerollData, ShinyData},
    types::ItemType,
};

use super::{error::ItemConvertError, GenericItem};

/// Struct Representing an Gear Item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct GearItem {
    /// Name of the gear item
    pub name: String,

    /// Identifications of the gear item
    ///
    /// If no identifications are present, the item is not identified and should be displayed as such
    pub identifications: Option<IdentificationData>,
    /// Powders of the gear item
    pub powders: Option<PowderData>,
    /// Shiny of the gear item
    pub shiny: Option<ShinyData>,
    /// Rerolls of the gear item
    pub rerolls: Option<RerollData>,
}

impl TryFrom<GenericItem> for GearItem {
    type Error = ItemConvertError;

    fn try_from(value: GenericItem) -> Result<Self, Self::Error> {
        if let ItemType::Gear = value.kind {
            Ok(Self {
                name: value
                    .name
                    .ok_or(ItemConvertError::MissingField("name".to_string()))?,
                identifications: value.identifications,
                powders: value.powders,
                shiny: value.shiny,
                rerolls: value.rerolls,
            })
        } else {
            Err(ItemConvertError::InvalidItemType(value.kind))
        }
    }
}

impl From<GearItem> for GenericItem {
    fn from(value: GearItem) -> Self {
        Self {
            kind: ItemType::Gear,
            name: Some(value.name),
            identifications: value.identifications,
            powders: value.powders,
            shiny: value.shiny,
            rerolls: value.rerolls,
            ..Default::default()
        }
    }
}

/// Struct Representing a Tome Item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct TomeItem {
    /// Name of the tome item
    pub name: String,

    /// Identifications of the tome item
    ///
    /// If no identifications are present, the item is not identified and should be displayed as such
    pub identifications: Option<IdentificationData>,
    /// Rerolls of the tome item
    pub rerolls: Option<RerollData>,
}

impl TryFrom<GenericItem> for TomeItem {
    type Error = ItemConvertError;

    fn try_from(value: GenericItem) -> Result<Self, Self::Error> {
        if let ItemType::Tome = value.kind {
            Ok(Self {
                name: value
                    .name
                    .ok_or(ItemConvertError::MissingField("name".to_string()))?,
                identifications: value.identifications,
                rerolls: value.rerolls,
            })
        } else {
            Err(ItemConvertError::InvalidItemType(value.kind))
        }
    }
}

impl From<TomeItem> for GenericItem {
    fn from(value: TomeItem) -> Self {
        Self {
            kind: ItemType::Tome,
            name: Some(value.name),
            identifications: value.identifications,
            rerolls: value.rerolls,
            ..Default::default()
        }
    }
}

/// Struct Representing a Charm Item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CharmItem {
    /// Name of the charm item
    pub name: String,

    /// Identifications of the charm item
    ///
    /// If no identifications are present, the item is not identified and should be displayed as such
    pub identifications: Option<IdentificationData>,
    /// Rerolls of the charm item
    pub rerolls: Option<RerollData>,
}

impl TryFrom<GenericItem> for CharmItem {
    type Error = ItemConvertError;

    fn try_from(value: GenericItem) -> Result<Self, Self::Error> {
        if let ItemType::Charm = value.kind {
            Ok(Self {
                name: value
                    .name
                    .ok_or(ItemConvertError::MissingField("name".to_string()))?,
                identifications: value.identifications,
                rerolls: value.rerolls,
            })
        } else {
            Err(ItemConvertError::InvalidItemType(value.kind))
        }
    }
}

impl From<CharmItem> for GenericItem {
    fn from(value: CharmItem) -> Self {
        Self {
            kind: ItemType::Charm,
            name: Some(value.name),
            identifications: value.identifications,
            rerolls: value.rerolls,
            ..Default::default()
        }
    }
}
