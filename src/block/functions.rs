use crate::{
    encoding::{
        self,
        string::{decode_string, encode_string},
    },
    types::EncodingVersion,
};

use super::AnyBlock;

/// Decode a full idstring into a list of blocks
///
/// This function performs a full decode of the given idstring and returns a list of blocks contained within it.
/// This function will return an error if the idstring is invalid and may error if the data within the idstring is invalid.
///
/// Format version is decoded from the start of the idstring.
///
/// For decoding an already decoded byte stream, use [`AnyBlock::decode`] instead.
pub fn decode_str(input: impl AsRef<str>) -> Result<Vec<AnyBlock>, encoding::DecoderError> {
    AnyBlock::decode(
        &mut decode_string(input)
            .map_err(|e| encoding::DecoderError {
                error: e.into(),
                during: None,
            })?
            .into_iter(),
    )
}

/// Encode a list of blocks into an idstring bytes
///
/// This function encodes a list of blocks into idstring bytes. The blocks are encoded in the order they are given.
/// This function does not validate the blocks, their order, or their contents.
///
/// The given version is used but the user is responsible for ensuring the version is correct and the same as set in the start block.
///
/// A valid idstring must start with a start block and end with an end block.
pub fn encode_blocks(
    ver: EncodingVersion,
    blocks: &[AnyBlock],
) -> Result<Vec<u8>, encoding::EncoderError> {
    let mut bytes = Vec::new();

    for block in blocks {
        block.encode(ver, &mut bytes)?;
    }

    Ok(bytes)
}

/// Encode a list of blocks into an idstring
///
/// This function encodes a list of blocks into an idstring. The blocks are encoded in the order they are given.
/// This function does not validate the blocks, their order, or their contents.
///
/// The given version is used but the user is responsible for ensuring the version is correct and the same as set in the start block.
///
/// A valid idstring must start with a start block and end with an end block.
pub fn encode_blocks_str(
    ver: EncodingVersion,
    blocks: &[AnyBlock],
) -> Result<String, encoding::EncoderError> {
    let bytes = encode_blocks(ver, blocks)?;

    Ok(encode_string(&bytes))
}
