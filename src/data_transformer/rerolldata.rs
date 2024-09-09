use crate::types::transform::TransformVersion;

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

#[derive(Debug, Clone)]
pub struct RerollData(pub u8);

impl TransformId for RerollData {
    fn get_id() -> u8 {
        DataTransformerTypes::RerollDataTransformer as u8
    }
}

impl DataEncoder for RerollData {
    fn encode_data(
        &self,
        ver: crate::types::transform::TransformVersion,
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
