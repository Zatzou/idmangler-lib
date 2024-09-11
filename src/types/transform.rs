use crate::DecodeError;

/// This enum represents the version of the encoding being used
///
/// At the current time the only version of the encoding is the first version
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub enum TransformVersion {
    /// Represents the version 1 of the wynntils encoding scheme
    Version1 = 0,
}

impl TransformVersion {
    pub fn version(&self) -> u8 {
        *self as u8
    }
}

impl TryFrom<u8> for TransformVersion {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Version1),
            _ => Err(DecodeError::UnknownVersion(value)),
        }
    }
}
