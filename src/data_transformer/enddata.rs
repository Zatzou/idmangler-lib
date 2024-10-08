use crate::types::TransformVersion;

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for the end data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct EndData;

impl TransformId for EndData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::EndData as u8;
}

impl DataEncoder for EndData {
    fn encode_data(&self, _ver: TransformVersion, _out: &mut Vec<u8>) -> Result<(), EncodeError> {
        // end data is always empty
        Ok(())
    }
}

impl DataDecoder for EndData {
    fn decode_data(
        _bytes: &mut impl Iterator<Item = u8>,
        _ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        // end data is always empty
        Ok(Self)
    }
}

impl From<EndData> for AnyData {
    fn from(value: EndData) -> Self {
        AnyData::EndData(value)
    }
}
