use crate::types::TransformVersion;

use super::{
    DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct UsesData {
    pub current: u8,
    pub max: u8,
}

impl TransformId for UsesData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::UsesData as u8;
}

impl DataEncoder for UsesData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                // first the current amount left
                out.push(self.current);
                // then the max amount
                out.push(self.max);

                Ok(())
            }
        }
    }
}

impl DataDecoder for UsesData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let current = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let max = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self { current, max })
            }
        }
    }
}

impl From<UsesData> for super::AnyData {
    fn from(value: UsesData) -> Self {
        Self::UsesData(value)
    }
}
