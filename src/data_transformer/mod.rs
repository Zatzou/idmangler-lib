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

/// Trait for providing the id of the transformer
pub trait TransformId {
    /// The id of this transformer
    const TRANSFORMER_ID: u8;
}

/// Trait for using a transformer to encode data into bytes
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
pub trait DataDecoder<B: Iterator<Item = u8>>: TransformId {
    /// Decode the data from a given byte stream
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

/// Function for fully decoding an idstring given its bytes.
pub fn decode<B: Iterator<Item = u8>>(bytes: &mut B) -> Result<Vec<AnyData>, DecodeError> {
    let mut out = Vec::new();

    // decode the start byte and version
    let ver = StartData::decode_start_bytes(bytes)?;

    while let Some(id) = bytes.next() {
        match id {
            0 => return Err(DecodeError::StartReparse),
            1 => out.push(AnyData::TypeData(TypeData::decode_data(bytes, ver)?)),
            2 => out.push(AnyData::NameData(NameData::decode_data(bytes, ver)?)),
            3 => out.push(AnyData::IdentificationData(
                IdentificationData::decode_data(bytes, ver)?,
            )),
            // TODO: powder decode
            5 => out.push(AnyData::RerollData(RerollData::decode_data(bytes, ver)?)),
            // TODO
            255 => out.push(AnyData::EndData(EndData::decode_data(bytes, ver)?)),
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
    InvalidType(u8),

    /// The decoder unexpectedly ran out of bytes to decode while decoding
    #[error("Unexpectedly hit end of bytestream while decoding")]
    UnexpectedEndOfBytes,
}

/// Enum representing the ids of the transformers
pub enum DataTransformerTypes {
    StartDataTransformer = 0,
    TypeDataTransformer = 1,
    NameDataTransformer = 2,
    IdentificationDataTransformer = 3,
    PowderDataTransformer = 4,
    RerollDataTransformer = 5,
    ShinyDataTransformer = 6,
    CustomGearTypeTransformer = 7,
    DurabilityDataTransformer = 8,
    RequirementsDataTransformer = 9,
    DamageDataTransformer = 10,
    DefenseDataTransformer = 11,
    CustomIdentificationDataTransformer = 12,
    CustomConsumableTypeDataTransformer = 13,
    UsesDataTransformer = 14,
    EffectsDataTransformer = 15,
    EndDataTransformer = 255,
}

#[derive(Debug)]
pub enum AnyData {
    StartData(StartData),
    TypeData(TypeData),
    NameData(NameData),
    IdentificationData(IdentificationData),
    PowderData(PowderData),
    RerollData(RerollData),
    // TODO
    EndData(EndData),
}
