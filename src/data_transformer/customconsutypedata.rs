use crate::types::{ConsumableType, TransformVersion};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CustomConsumableTypeData(pub ConsumableType);

impl TransformId for CustomConsumableTypeData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::CustomConsumableTypeData as u8;
}

impl DataEncoder for CustomConsumableTypeData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                out.push(self.0.into());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CustomConsumableTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let kind = ConsumableType::try_from(
                    bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?,
                )?;
                Ok(Self(kind))
            }
        }
    }
}

impl From<CustomConsumableTypeData> for AnyData {
    fn from(value: CustomConsumableTypeData) -> Self {
        Self::CustomConsumableTypeData(value)
    }
}
