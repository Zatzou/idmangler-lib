use crate::types::TransformVersion;

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for reroll data
#[derive(Debug, Clone)]
pub struct RerollData(pub u8);

impl TransformId for RerollData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::RerollDataTransformer as u8;
}

impl DataEncoder for RerollData {
    fn encode_data(
        &self,
        ver: crate::types::TransformVersion,
        out: &mut Vec<u8>,
    ) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => out.push(self.0),
        }

        Ok(())
    }
}

impl<B: Iterator<Item = u8>> DataDecoder<B> for RerollData {
    fn decode_data(bytes: &mut B, ver: TransformVersion) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                Ok(Self(bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?))
            }
        }
    }
}
