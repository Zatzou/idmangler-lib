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
pub struct DefenseData {
    /// Amount of health this item grants
    health: i32,
    /// Defenses against elements
    defences: Vec<(Element, i32)>,
}

impl BlockId for DefenseData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::DefenseData
    }
}

impl DataEncoder for DefenseData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                // health value
                out.append(&mut encode_varint(self.health as i64));

                if self.defences.len() > 255 {
                    return Err(EncodeError::TooManyDefences);
                }

                // number of defences
                out.push(self.defences.len() as u8);

                for (element, value) in self.defences.iter() {
                    // element id
                    out.push((*element).into());

                    // defence value
                    out.append(&mut encode_varint(*value as i64));
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
            EncodingVersion::Version1 => {
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
