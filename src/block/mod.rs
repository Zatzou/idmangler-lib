//! Module implementing the data blocks for the encoding of the idstrings

pub mod anyblock;

mod customconsutypedata;
use anyblock::AnyBlock;
#[doc(inline)]
pub use customconsutypedata::CraftedConsumableTypeData;

mod customgeartypedata;
#[doc(inline)]
pub use customgeartypedata::CraftedGearTypeData;

mod customidentdata;
#[doc(inline)]
pub use customidentdata::CraftedIdentificationData;

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
use thiserror::Error;
#[doc(inline)]
pub use typedata::TypeData;

mod usesdata;
#[doc(inline)]
pub use usesdata::UsesData;

use crate::{
    encoding::{DataDecoder, DecodeError, DecoderError},
    types::EncodingVersion,
};

/// Enum representing the ids of the blocks
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBlockId {
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

impl DataBlockId {
    pub fn decode(
        &self,
        ver: EncodingVersion,
        bytes: &mut impl Iterator<Item = u8>,
    ) -> Result<AnyBlock, DecoderError> {
        #[allow(clippy::needless_late_init)]
        let out: AnyBlock;
        match self {
            DataBlockId::TypeData => {
                out = Box::new(TypeData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::NameData => {
                out = Box::new(NameData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::IdentificationData => {
                out = Box::new(
                    IdentificationData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::PowderData => {
                out = Box::new(PowderData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::RerollData => {
                out = Box::new(RerollData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::ShinyData => {
                out = Box::new(ShinyData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::CustomGearType => {
                out = Box::new(
                    CraftedGearTypeData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::DurabilityData => {
                out = Box::new(
                    DurabilityData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::RequirementsData => {
                out = Box::new(
                    RequirementsData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::DamageData => {
                out = Box::new(DamageData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::DefenseData => {
                out = Box::new(DefenseData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::CustomIdentificationData => {
                out = Box::new(
                    CraftedIdentificationData::decode_data(bytes, ver)
                        .map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::CustomConsumableTypeData => {
                out = Box::new(
                    CraftedConsumableTypeData::decode_data(bytes, ver)
                        .map_err(|e| err_map(e, *self))?,
                )
            }
            DataBlockId::UsesData => {
                out = Box::new(UsesData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::EffectsData => {
                out = Box::new(EffectsData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::EndData => {
                out = Box::new(EndData::decode_data(bytes, ver).map_err(|e| err_map(e, *self))?)
            }
            DataBlockId::StartData => {
                return Err(err_map(DecodeError::StartReparse, *self));
            }
        };

        Ok(out)
    }
}

fn err_map(e: DecodeError, id: DataBlockId) -> DecoderError {
    DecoderError {
        error: e,
        during: Some(id),
    }
}

impl From<DataBlockId> for u8 {
    fn from(id: DataBlockId) -> u8 {
        id as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid block id: {0}")]
pub struct InvalidBlockId(pub u8);

impl TryFrom<u8> for DataBlockId {
    type Error = InvalidBlockId;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DataBlockId::StartData),
            1 => Ok(DataBlockId::TypeData),
            2 => Ok(DataBlockId::NameData),
            3 => Ok(DataBlockId::IdentificationData),
            4 => Ok(DataBlockId::PowderData),
            5 => Ok(DataBlockId::RerollData),
            6 => Ok(DataBlockId::ShinyData),
            7 => Ok(DataBlockId::CustomGearType),
            8 => Ok(DataBlockId::DurabilityData),
            9 => Ok(DataBlockId::RequirementsData),
            10 => Ok(DataBlockId::DamageData),
            11 => Ok(DataBlockId::DefenseData),
            12 => Ok(DataBlockId::CustomIdentificationData),
            13 => Ok(DataBlockId::CustomConsumableTypeData),
            14 => Ok(DataBlockId::UsesData),
            15 => Ok(DataBlockId::EffectsData),
            255 => Ok(DataBlockId::EndData),
            _ => Err(InvalidBlockId(value)),
        }
    }
}
