use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        AnyData, BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{CraftedStat, EncodingVersion},
};

use super::DataBlockId;

/// Identifications of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CraftedIdentificationData {
    pub idents: Vec<CraftedStat>,
}

impl BlockId for CraftedIdentificationData {
    const BLOCK_ID: u8 = DataBlockId::CustomIdentificationData as u8;
}

impl DataEncoder for CraftedIdentificationData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
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

impl DataDecoder for CraftedIdentificationData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                // ident count
                let count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let mut idents = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    // type of ident
                    let kind = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                    // value of ident
                    let max = decode_varint(bytes)? as i32;

                    idents.push(CraftedStat { kind, max });
                }

                Ok(Self { idents })
            }
        }
    }
}

impl From<CraftedIdentificationData> for AnyData {
    fn from(value: CraftedIdentificationData) -> Self {
        Self::CustomIdentificationData(value)
    }
}
