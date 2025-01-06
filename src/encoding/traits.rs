use crate::{
    block::{AnyBlock, DataBlockId},
    types::EncodingVersion,
};

use super::{DecodeError, EncodeError, EncoderError};

/// Trait for providing the id of the block
pub trait BlockId {
    /// The id of this block
    fn block_id(&self) -> DataBlockId;
}

/// Trait for encoding data into bytes
#[allow(private_bounds)]
pub trait DataEncoder: BlockId {
    /// Function for encoding the full data block of this data
    fn encode(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncoderError> {
        // skip encoding data which should not be encoded
        if !self.should_encode_data(ver) {
            return Ok(());
        }

        // encode the id
        out.push(u8::from(self.block_id()));

        // encode the data
        self.encode_data(ver, out).map_err(|e| EncoderError {
            error: e,
            during: self.block_id(),
        })?;

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
pub trait DataDecoder: BlockId + Into<AnyBlock> {
    /// Decode the data from a given byte stream
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}
