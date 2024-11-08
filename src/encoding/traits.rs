use crate::types::EncodingVersion;

use super::{AnyData, DecodeError, EncodeError};

/// Trait for providing the id of the transformer
pub(crate) trait TransformId {
    /// The id of this transformer
    const TRANSFORMER_ID: u8;
}

/// Trait for encoding data into bytes
#[allow(private_bounds)]
pub trait DataEncoder: TransformId {
    /// Function for encoding the full data block of this data
    fn encode(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        // skip encoding data which should not be encoded
        if !self.should_encode_data(ver) {
            return Ok(());
        }

        // encode the id
        out.push(Self::TRANSFORMER_ID);

        // encode the data
        self.encode_data(ver, out)?;

        Ok(())
    }

    /// Function for encoding the payload of this data
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError>;

    /// Whether or not this encoder should actually encode anything
    fn should_encode_data(&self, _ver: EncodingVersion) -> bool {
        true
    }
}

/// Trait for decoding data from bytes
#[allow(private_bounds)]
pub trait DataDecoder: TransformId + Into<AnyData> {
    /// Decode the data from a given byte stream
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}
