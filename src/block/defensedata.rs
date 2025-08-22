use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{Element, EncodingVersion},
};

use super::{AnyBlock, DataBlockId};

/// Defense values of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefenseData {
    /// Amount of health this item grants
    pub health: i32,
    /// Defenses against elements
    pub defences: Vec<(Element, i32)>,
}

impl BlockId for DefenseData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::DefenseData
    }
}

impl DataEncoder for DefenseData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                // health value
                out.append(&mut encode_varint(self.health));

                let def_len =
                    u8::try_from(self.defences.len()).map_err(|_| EncodeError::TooManyDefences)?;

                // number of defences
                out.push(def_len);

                for (element, value) in &self.defences {
                    // element id
                    out.push((*element).into());

                    // defence value
                    out.append(&mut encode_varint(*value));
                }
            }
        }

        todo!()
    }

    fn should_encode_data(&self, _ver: EncodingVersion) -> bool {
        self.health != 0 || !self.defences.is_empty()
    }
}

impl DataDecoder for DefenseData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 | EncodingVersion::V2 => {
                // health value
                let health = decode_varint(bytes)? as i32;

                // number of defences
                let num_defences = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)? as usize;

                let mut defences = Vec::with_capacity(num_defences);

                for _ in 0..num_defences {
                    // element id
                    let element =
                        Element::try_from(bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?)?;

                    // defence value
                    let value = decode_varint(bytes)? as i32;

                    defences.push((element, value));
                }

                Ok(Self { health, defences })
            }
        }
    }
}

impl From<DefenseData> for AnyBlock {
    fn from(data: DefenseData) -> Self {
        AnyBlock::DefenseData(data)
    }
}
