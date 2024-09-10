use crate::types::transform::TransformVersion;

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

#[derive(Debug, Clone)]
pub struct EndData;

impl TransformId for EndData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::EndDataTransformer as u8;
}

impl DataEncoder for EndData {
    fn encode_data(&self, _ver: TransformVersion, _out: &mut Vec<u8>) -> Result<(), EncodeError> {
        // end data is always empty
        Ok(())
    }
}

impl<B: Iterator<Item = u8>> DataDecoder<B> for EndData {
    fn decode_data(_bytes: &mut B, _ver: TransformVersion) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        // end data is always empty
        Ok(Self)
    }
}
