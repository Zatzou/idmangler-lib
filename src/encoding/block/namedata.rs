use crate::{
    encoding::{
        traits::{DataDecoder, DataEncoder, TransformId},
        AnyData, DecodeError, EncodeError,
    },
    types::EncodingVersion,
};

use super::DataBlockId;

/// The transformer for item name data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct NameData(pub String);

impl TransformId for NameData {
    const TRANSFORMER_ID: u8 = DataBlockId::NameData as u8;
}

impl DataEncoder for NameData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                // check that the string is valid ascii
                if !self.0.is_ascii() {
                    return Err(EncodeError::NonAsciiString);
                }

                // push the bytes
                out.extend_from_slice(self.0.as_bytes());
                // push the null terminator
                out.push(0);
            }
        }

        Ok(())
    }
}

impl DataDecoder for NameData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let b: Vec<u8> = bytes.take_while(|b| *b != 0).collect();

                // UTF-8 and ASCII share the same set of characters
                Ok(NameData(
                    String::from_utf8(b).map_err(|_| DecodeError::BadString)?,
                ))
            }
        }
    }
}

impl From<NameData> for AnyData {
    fn from(value: NameData) -> Self {
        Self::NameData(value)
    }
}
