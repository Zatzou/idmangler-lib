use crate::types::TransformVersion;

use super::{DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId};

/// The start data of the encoding. The start data holds the version of the encoding to be used
#[derive(Debug, Clone)]
pub struct StartData(pub TransformVersion);

impl TransformId for StartData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::StartData as u8;
}

impl DataEncoder for StartData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => out.push(self.0.version()),
        }

        Ok(())
    }
}

impl StartData {
    /// Special case function for parsing the start bytes
    pub(crate) fn decode_start_bytes<B: Iterator<Item = u8>>(
        bytes: &mut B,
    ) -> Result<TransformVersion, DecodeError> {
        let idbyte = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

        if idbyte != DataTransformerTypes::StartData as u8 {
            return Err(DecodeError::NoStartBlock);
        }

        let verbyte = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

        TransformVersion::try_from(verbyte)
    }
}
