use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{CraftedGearType, EncodingVersion},
};

use super::DataBlockId;

/// Sets the gear type of a crafted item
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CraftedGearTypeData(pub CraftedGearType);

impl BlockId for CraftedGearTypeData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::CustomGearType
    }
}

impl DataEncoder for CraftedGearTypeData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                out.push(self.0.get_encode_id());
                Ok(())
            }
        }
    }
}

impl DataDecoder for CraftedGearTypeData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let gear = CraftedGearType::try_from(id)?;
                Ok(Self(gear))
            }
        }
    }
}
