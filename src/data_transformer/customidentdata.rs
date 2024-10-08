use crate::{
    encoding::encode_varint,
    types::{CustomStat, TransformVersion},
};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// Identifications of a crafted item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct CustomIdentificationData {
    pub idents: Vec<CustomStat>,
}

impl TransformId for CustomIdentificationData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::CustomIdentificationData as u8;
}

impl DataEncoder for CustomIdentificationData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                if self.idents.len() > 255 {
                    return Err(EncodeError::TooManyIdentifications);
                }

                // number of idents
                out.push(self.idents.len() as u8);

                for ident in self.idents.iter() {
                    // ident id
                    out.push(ident.kind);

                    // ident value
                    out.append(&mut encode_varint(ident.max as i64));
                }

                Ok(())
            }
        }
    }
}

impl DataDecoder for CustomIdentificationData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                // ident count
                let count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let mut idents = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    // type of ident
                    let kind = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                    // value of ident
                    let max = crate::encoding::decode_varint(bytes)? as i32;

                    idents.push(CustomStat { kind, max });
                }

                Ok(Self { idents })
            }
        }
    }
}

impl From<CustomIdentificationData> for AnyData {
    fn from(value: CustomIdentificationData) -> Self {
        Self::CustomIdentificationData(value)
    }
}
