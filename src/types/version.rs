use thiserror::Error;

/// This enum represents the version of the encoding being used
///
/// At the current time the only version of the encoding is the first version
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum EncodingVersion {
    /// Represents the version 1 of the wynntils encoding scheme
    Version1 = 0,
}

impl EncodingVersion {
    pub fn version(&self) -> u8 {
        *self as u8
    }
}

#[derive(Error, Debug)]
#[error("Unknown encoding version: {0}")]
pub struct UnknownEncodingVersion(pub u8);

impl TryFrom<u8> for EncodingVersion {
    type Error = UnknownEncodingVersion;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Version1),
            _ => Err(UnknownEncodingVersion(value)),
        }
    }
}
