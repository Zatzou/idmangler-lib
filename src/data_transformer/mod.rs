//! The main module for transforming data between structs and the encoded format

use thiserror::Error;

use crate::types::TransformVersion;

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

mod customtypedata;
#[doc(inline)]
pub use customtypedata::CustomTypeData;

mod durabilitydata;
#[doc(inline)]
pub use durabilitydata::DurabilityData;

/// Trait for providing the id of the transformer
pub(crate) trait TransformId {
    /// The id of this transformer
    const TRANSFORMER_ID: u8;
}

/// Trait for encoding data into bytes
#[allow(private_bounds)]
pub trait DataEncoder: TransformId {
    /// Function for encoding the full data block of this data
    fn encode(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        // skip encoding data which should not be encoded
        if !self.should_encode_data(ver) {
            return Ok(());
        }

        // encode the id
        out.push(Self::TRANSFORMER_ID);

        // encode the data
        self.encode_data(ver, out)?;

        Ok(())
    }

    /// Function for encoding the payload of this data
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError>;

    /// Whether or not this encoder should actually encode anything
    fn should_encode_data(&self, _ver: TransformVersion) -> bool {
        true
    }
}

/// Trait for decoding data from bytes
#[allow(private_bounds)]
pub trait DataDecoder: TransformId + Into<AnyData> {
    /// Decode the data from a given byte stream
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

/// Decode the bytes contained within an idstring
pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<AnyData>, DecodeError> {
    let mut out = Vec::new();

    let mut iter = bytes.iter().copied();
    let bytes = &mut iter;

    // decode the start byte and version
    let ver = StartData::decode_start_bytes(bytes)?;

    // push the start data to the output
    out.push(AnyData::StartData(StartData(ver)));

    while let Some(id) = bytes.next() {
        match id {
            0 => return Err(DecodeError::StartReparse),
            1 => out.push(TypeData::decode_data(bytes, ver)?.into()),
            2 => out.push(NameData::decode_data(bytes, ver)?.into()),
            3 => out.push(IdentificationData::decode_data(bytes, ver)?.into()),
            4 => out.push(PowderData::decode_data(bytes, ver)?.into()),
            5 => out.push(RerollData::decode_data(bytes, ver)?.into()),
            6 => out.push(ShinyData::decode_data(bytes, ver)?.into()),

            7 => out.push(CustomTypeData::decode_data(bytes, ver)?.into()),
            8 => out.push(DurabilityData::decode_data(bytes, ver)?.into()),
            // TODO
            255 => out.push(EndData::decode_data(bytes, ver)?.into()),
            _ => return Err(DecodeError::UnknownTransformer(id)),
        }
    }

    Ok(out)
}

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

    /// Effect strength should be a percentage between 0 and 100
    #[error("Effect strength of {0} is too high, it should be a percentage between 0 and 100")]
    EffectStrengthTooHigh(u8),
}

/// Potential errors thrown while decoding id strings
#[derive(Error, Debug)]
pub enum DecodeError {
    /// The idstring does not start with a valid start block
    #[error("No start block found")]
    NoStartBlock,
    /// Encoding of an unknown potentially future version was hit
    #[error("Unknown encoding version: `{0}`")]
    UnknownVersion(u8),
    /// Decoder found a second start block in the data
    #[error("Second start block found in data")]
    StartReparse,
    /// Decoder hit an unknown transformer which it could not decode
    #[error("Unknown transformer id:`{0}` was found")]
    UnknownTransformer(u8),

    /// An invalid non ascii/utf-8 string was decoded by the parser
    #[error("Decoder decoded a bad string")]
    BadString,
    /// An invalid type was found
    #[error("Invalid type of id:`{0}` was decoded")]
    InvalidUItemType(u8),

    /// An invalid powder was encountered
    #[error("Invalid powder of id:`{0}` was decoded")]
    InvalidPowder(u8),

    #[error("Invalid gear type id:`{0}` was decoded")]
    BadGearType(u8),

    /// The decoder unexpectedly ran out of bytes to decode while decoding
    #[error("Unexpectedly hit end of bytestream while decoding")]
    UnexpectedEndOfBytes,
}

/// Enum representing the ids of the transformers
#[allow(unused)] // TODO: implement rest of the transformers
enum DataTransformerTypes {
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

/// Represents any possible item data type
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum AnyData {
    StartData(StartData),
    TypeData(TypeData),
    NameData(NameData),
    IdentificationData(IdentificationData),
    PowderData(PowderData),
    RerollData(RerollData),
    ShinyData(ShinyData),

    CustomTypeData(CustomTypeData),
    DurabilityData(DurabilityData),
    // TODO
    EndData(EndData),
}
