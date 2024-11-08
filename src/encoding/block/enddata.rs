use crate::{
    encoding::{
        traits::{BlockId, DataDecoder, DataEncoder},
        AnyData, DecodeError, EncodeError,
    },
    types::EncodingVersion,
};

use super::DataBlockId;

/// The block for the end data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct EndData;

impl BlockId for EndData {
    const BLOCK_ID: u8 = DataBlockId::EndData as u8;
}

impl DataEncoder for EndData {
    fn encode_data(&self, _ver: EncodingVersion, _out: &mut Vec<u8>) -> Result<(), EncodeError> {
        // end data is always empty
        Ok(())
    }
}

impl DataDecoder for EndData {
    fn decode_data(
        _bytes: &mut impl Iterator<Item = u8>,
        _ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        // end data is always empty
        Ok(Self)
    }
}

impl From<EndData> for AnyData {
    fn from(value: EndData) -> Self {
        AnyData::EndData(value)
    }
}
