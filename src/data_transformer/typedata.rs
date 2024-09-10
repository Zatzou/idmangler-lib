use crate::types::{ItemType, TransformVersion};

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for the item type data
#[derive(Debug, Clone)]
pub struct TypeData(pub ItemType);

impl TransformId for TypeData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::TypeDataTransformer as u8;
}

impl DataEncoder for TypeData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => out.push(self.0.into()),
        }

        Ok(())
    }
}

impl<B: Iterator<Item = u8>> DataDecoder<B> for TypeData {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let b = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self(
                    ItemType::try_from(b).map_err(|_| DecodeError::InvalidType(b))?,
                ))
            }
        }
    }
}
