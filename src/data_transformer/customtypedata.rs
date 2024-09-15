use crate::types::{GearType, TransformVersion};

use super::{AnyData, DataDecoder, DataEncoder, DecodeError, EncodeError, TransformId};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CustomTypeData(pub GearType);

impl TransformId for CustomTypeData {
    const TRANSFORMER_ID: u8 = 7;
}

impl DataEncoder for CustomTypeData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                out.push(self.0.get_encode_id());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CustomTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let gear = GearType::try_from(id)?;
                Ok(Self(gear))
            }
        }
    }
}

impl From<CustomTypeData> for AnyData {
    fn from(data: CustomTypeData) -> Self {
        AnyData::CustomTypeData(data)
    }
}
