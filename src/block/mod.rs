//! Module implementing the data blocks for the encoding of the idstrings
//!
//! This module contains the definitions for the data blocks used in the encoding of the idstrings.
//! These blocks are used to represent the data of the items in the idstrings.
//!
//! Blocks can be encoded and decoded using the functions provided in this module and the methods provided by [`AnyBlock`] and [`DataBlockId`].
//!
//! In general the item module of this crate should be used for decoding and encoding items when low level block manipulation is not required.

mod functions;
#[doc(inline)]
pub use functions::*;

mod craftedconsutypedata;
#[doc(inline)]
pub use craftedconsutypedata::CraftedConsumableTypeData;

mod craftedgeartypedata;
#[doc(inline)]
pub use craftedgeartypedata::CraftedGearTypeData;

mod craftedidentdata;
#[doc(inline)]
pub use craftedidentdata::CraftedIdentificationData;

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
    encoding::{DataDecoder, DataEncoder, DecodeError, DecoderError, EncoderError},
    types::EncodingVersion,
};

/// Macro for defining the enums for dealing with the data blocks
///
/// This macro will define the data block id enum and the any block enum for the given data blocks.
/// This mainly saves a lot of work writing these manually.
macro_rules! datablock_defs {
    (
        $(($name:ident, $id:expr, $ty:ty),)+
    ) => {
        /// Enum representing the ids of the data blocks
        #[repr(u8)]
        #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
        pub enum DataBlockId {
            $(
                $name = $id,
            )+
        }

        #[derive(Error, Debug)]
        #[error("Invalid block id: {0}")]
        pub struct InvalidBlockId(pub u8);

        impl TryFrom<u8> for DataBlockId {
            type Error = InvalidBlockId;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $id => Ok(DataBlockId::$name),
                    )+
                    _ => Err(InvalidBlockId(value)),
                }
            }
        }

        impl DataBlockId {
            /// Try to decode a block with the type of this block id
            pub fn decode(&self, ver: EncodingVersion, bytes: &mut impl Iterator<Item = u8>) -> Result<AnyBlock, DecoderError> {
                Ok(match self {
                    $(
                        DataBlockId::$name => self.decode_with::<$ty>(bytes, ver)?.into(),
                    )+
                })
            }
        }

        /// Enum representing any of the data blocks
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum AnyBlock {
            $(
                $name($ty),
            )+
        }

        impl AnyBlock {
            /// Encode this block into the given output buffer
            ///
            /// This will encode the block id and the data of the block and append it to the output buffer
            pub fn encode(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncoderError> {
                match self {
                    $(
                        AnyBlock::$name(data) => data.encode(ver, out),
                    )+
                }
            }

            /// Get the id of this block without consuming it
            pub fn as_id(&self) -> DataBlockId {
                match self {
                    $(
                        AnyBlock::$name(_) => DataBlockId::$name,
                    )+
                }
            }
        }
    };
}

datablock_defs! {
    (StartData, 0, StartData),
    (TypeData, 1, TypeData),
    (NameData, 2, NameData),
    (IdentificationData, 3, IdentificationData),
    (PowderData, 4, PowderData),
    (RerollData, 5, RerollData),
    (ShinyData, 6, ShinyData),
    (CraftedGearType, 7, CraftedGearTypeData),
    (DurabilityData, 8, DurabilityData),
    (RequirementsData, 9, RequirementsData),
    (DamageData, 10, DamageData),
    (DefenseData, 11, DefenseData),
    (CraftedIdentificationData, 12, CraftedIdentificationData),
    (CraftedConsumableTypeData, 13, CraftedConsumableTypeData),
    (UsesData, 14, UsesData),
    (EffectsData, 15, EffectsData),
    (EndData, 255, EndData),
}

// do impls that dont need the macro outside of it to make rust-analyzer happy
impl From<DataBlockId> for u8 {
    fn from(id: DataBlockId) -> u8 {
        id as u8
    }
}

impl DataBlockId {
    /// Attempt to decode a block with assumed type using the given decoder
    fn decode_with<T: DataDecoder>(
        &self,
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<T, DecoderError> {
        T::decode_data(bytes, ver).map_err(|e| DecoderError {
            error: e,
            during: Some(*self),
        })
    }
}

// anyblock impls that dont need the macro
impl From<AnyBlock> for DataBlockId {
    fn from(block: AnyBlock) -> DataBlockId {
        block.as_id()
    }
}

impl From<AnyBlock> for u8 {
    fn from(block: AnyBlock) -> u8 {
        DataBlockId::from(block) as u8
    }
}

impl AnyBlock {
    /// Decode a block from the given byte stream
    ///
    /// This will read the block id and then decode the data of the block
    pub fn decode_one(
        ver: EncodingVersion,
        bytes: &mut impl Iterator<Item = u8>,
    ) -> Result<Self, DecoderError> {
        // read the id of the block
        let id = bytes.next().ok_or(DecoderError {
            error: DecodeError::UnexpectedEndOfBytes,
            during: None,
        })?;
        let block_id = DataBlockId::try_from(id).map_err(|e| DecoderError {
            error: DecodeError::UnknownBlock(e),
            during: None,
        })?;

        // decode using the decoder for the block id
        block_id.decode(ver, bytes)
    }

    /// Decode all blocks from the given byte stream
    ///
    /// This will read blocks from the byte stream until the end block is reached
    pub fn decode_all(
        ver: EncodingVersion,
        bytes: &mut impl Iterator<Item = u8>,
    ) -> Result<Vec<Self>, DecoderError> {
        let mut blocks = Vec::new();
        let mut cont = true;
        while cont {
            // read block
            let block = Self::decode_one(ver, bytes)?;

            // if we reached the end block, stop
            if let AnyBlock::EndData(_) = block {
                cont = false;
            }

            blocks.push(block);
        }
        Ok(blocks)
    }

    /// Decode a fully formed idstring from the given byte stream
    ///
    /// This function assumes that the byte stream is a valid idstring which starts with a start block and ends with an end block
    pub fn decode(bytes: &mut impl Iterator<Item = u8>) -> Result<Vec<Self>, DecoderError> {
        // read the start data
        let start = StartData::decode_start_bytes(bytes).map_err(|e| DecoderError {
            error: e,
            during: Some(DataBlockId::StartData),
        })?;

        // create the output buffer with the start data so we also return the start data
        let mut out = vec![StartData(start).into()];

        // decode the rest of the blocks
        out.append(&mut Self::decode_all(start, bytes)?);

        Ok(out)
    }
}
