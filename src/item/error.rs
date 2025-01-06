use thiserror::Error;

use crate::{
    block::DataBlockId,
    encoding::{string::BadCodepoint, DecoderError},
    types::ItemType,
};

#[derive(Error, Debug)]
pub enum ItemConvertError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid item type: {0:?}")]
    InvalidItemType(ItemType),
}

#[derive(Error, Debug)]
pub enum ItemDecodeError {
    #[error("Decode error: {0:?}")]
    DecoderError(#[from] DecoderError),
    #[error("Missing data block: {0:?}")]
    MissingBlock(DataBlockId),
    #[error("Invalid codepoint encountered: {0}")]
    BadString(#[from] BadCodepoint),

    #[error("Invalid item: {0:?}")]
    InvalidItem(#[from] ItemConvertError),
}
