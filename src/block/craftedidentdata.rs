use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{CraftedStat, EncodingVersion},
};

use super::{AnyBlock, DataBlockId};

/// Identifications of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraftedIdentificationData {
    pub idents: Vec<CraftedStat>,
}

impl BlockId for CraftedIdentificationData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::CraftedIdentificationData
    }
}

impl DataEncoder for CraftedIdentificationData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 => {
                let ident_len = u8::try_from(self.idents.len())
                    .map_err(|_| EncodeError::TooManyIdentifications)?;

                // number of idents
                out.push(ident_len);

                for ident in &self.idents {
                    // ident id
                    out.push(ident.kind);

                    // ident value
                    out.append(&mut encode_varint(ident.max));
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
            EncodingVersion::V1 => {
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

impl From<CraftedIdentificationData> for AnyBlock {
    fn from(data: CraftedIdentificationData) -> Self {
        AnyBlock::CraftedIdentificationData(data)
    }
}
