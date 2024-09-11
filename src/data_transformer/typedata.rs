use crate::types::{ItemType, TransformVersion};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// The transformer for the item type data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Debug)]
pub struct TypeData(pub ItemType);

impl TransformId for TypeData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::TypeData as u8;
}

impl DataEncoder for TypeData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => out.push(self.0.into()),
        }

        Ok(())
    }
}

impl DataDecoder for TypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let b = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                Ok(Self(ItemType::try_from(b)?))
            }
        }
    }
}

impl From<TypeData> for AnyData {
    fn from(value: TypeData) -> Self {
        Self::TypeData(value)
    }
}
