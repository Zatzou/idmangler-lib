use crate::{
    encoding::{AnyData, BlockId, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

use super::DataBlockId;

/// The start data of the encoding. The start data holds the version of the encoding to be used
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct StartData(pub EncodingVersion);

impl BlockId for StartData {
    const BLOCK_ID: u8 = DataBlockId::StartData as u8;
}

impl DataEncoder for StartData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => out.push(self.0.version()),
        }

        Ok(())
    }
}

impl StartData {
    /// Special case function for parsing the start bytes
    pub(crate) fn decode_start_bytes<B: Iterator<Item = u8>>(
        bytes: &mut B,
    ) -> Result<EncodingVersion, DecodeError> {
        let idbyte = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

        if idbyte != DataBlockId::StartData as u8 {
            return Err(DecodeError::NoStartBlock);
        }

        let verbyte = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

        Ok(EncodingVersion::try_from(verbyte)?)
    }
}

impl From<StartData> for AnyData {
    fn from(value: StartData) -> Self {
        Self::StartData(value)
    }
}
