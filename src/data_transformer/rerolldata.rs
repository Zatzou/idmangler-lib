use crate::types::TransformVersion;

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for reroll data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct RerollData(pub u8);

impl TransformId for RerollData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::RerollData as u8;
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

impl DataDecoder for RerollData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
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

impl From<RerollData> for AnyData {
    fn from(value: RerollData) -> Self {
        Self::RerollData(value)
    }
}
