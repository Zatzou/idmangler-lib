use thiserror::Error;

use crate::types::errors::{
    BadAttackSpeed, BadClassType, BadConsumableType, BadEffectType, BadElement, BadGearType,
    BadItemType, BadSkillType, UnknownEncodingVersion,
};

/// Potential errors thrown during encoding of id strings
#[derive(Error, Debug)]
pub enum EncodeError {
    /// Encoder was given a string with non ascii characters.
    #[error("Cannot encode non ascii string")]
    NonAsciiString,

    /// More than 255 identifications were passed for encoding
    #[error("Cannot encode more than 255 identifications per item")]
    TooManyIdentifications,
    /// Identification is missing a basevalue while using the extended encoding scheme.
    ///
    /// An id is required to have an basevalue if the extended encoding is used for idents
    #[error("Identification id: {0} was not given a base value while using extended encoding")]
    NoBasevalueGiven(u8),

    /// More than 255 powders were passed for encoding
    #[error("Cannot encode more than 255 powders per item")]
    TooManyPowders,
    /// Invalid tier for a powder was passed
    #[error("Invalid powder tier of {0} was passed")]
    InvalidPowderTier(u8),

    /// Effect strength should be a percentage between 0 and 100
    #[error("Effect strength of {0} is too high, it should be a percentage between 0 and 100")]
    EffectStrengthTooHigh(u8),

    /// More than 255 skills were passed for encoding
    #[error("Cannot encode more than 255 skills per item")]
    TooManySkills,

    /// More than 255 damage values were passed for encoding
    #[error("Cannot encode more than 255 damage values per item")]
    TooManyDamageValues,

    /// More than 255 effects were passed for encoding
    #[error("Cannot encode more than 255 effects per item")]
    TooManyEffects,

    /// More than 255 defense values were passed for encoding
    #[error("Cannot encode more than 255 defense values per item")]
    TooManyDefences,
}

/// Potential errors thrown while decoding id strings
#[derive(Error, Debug)]
pub enum DecodeError {
    /// The idstring does not start with a valid start block
    #[error("No start block found")]
    NoStartBlock,
    /// Encoding of an unknown potentially future version was hit
    #[error("Unknown encoding version: `{}`", 0.0)]
    UnknownVersion(#[from] UnknownEncodingVersion),
    /// Decoder found a second start block in the data
    #[error("Second start block found in data")]
    StartReparse,
    /// Decoder hit an unknown block which it could not decode
    #[error("Unknown block id:`{0}` was found")]
    UnknownBlock(u8),

    /// An invalid non ascii/utf-8 string was decoded by the parser
    #[error("Decoder decoded a bad string")]
    BadString,
    /// An invalid type was found
    #[error("Invalid type of id:`{}` was decoded", 0.0)]
    BadItemType(#[from] BadItemType),

    #[error("Invalid gear type id:`{}` was decoded", 0.0)]
    BadGearType(#[from] BadGearType),

    /// An invalid class type was encountered
    #[error("Invalid class type id:`{}`", 0.0)]
    BadClassType(#[from] BadClassType),
    /// An invalid skill type was encountered
    #[error("Invalid skill type id:`{}`", 0.0)]
    BadSkillType(#[from] BadSkillType),

    /// An invalid attack speed was encountered
    #[error("Invalid attack speed id:`{}`", 0.0)]
    BadAttackSpeed(#[from] BadAttackSpeed),

    /// An invalid element id was encountered
    #[error("Invalid element id:`{}`", 0.0)]
    BadElement(#[from] BadElement),

    /// An invalid consumable type was encountered
    #[error("Invalid consumable type id:`{}`", 0.0)]
    BadConsumableType(#[from] BadConsumableType),

    /// An invalid effect type was encountered
    #[error("Invalid effect type id:`{}`", 0.0)]
    BadEffectType(#[from] BadEffectType),

    /// The decoder unexpectedly ran out of bytes to decode while decoding
    #[error("Unexpectedly hit end of bytestream while decoding")]
    UnexpectedEndOfBytes,
}
