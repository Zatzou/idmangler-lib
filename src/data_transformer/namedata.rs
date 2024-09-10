use crate::types::TransformVersion;

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for item name data
#[derive(Debug, Clone)]
pub struct NameData(pub String);

impl TransformId for NameData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::NameDataTransformer as u8;
}

impl DataEncoder for NameData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                // check that the string is valid ascii
                if self.0.chars().any(|c| !c.is_ascii()) {
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

impl<B: Iterator<Item = u8>> DataDecoder<B> for NameData {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let b: Vec<u8> = bytes.take_while(|b| *b != 0).collect();

                // UTF-8 and ASCII share the same set of characters
                Ok(NameData(
                    String::from_utf8(b).map_err(|_| DecodeError::BadString)?,
                ))
            }
        }
    }
}
