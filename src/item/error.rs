use thiserror::Error;

use crate::types::ItemType;

#[derive(Error, Debug)]
pub enum ItemConvertError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid item type: {0:?}")]
    InvalidItemType(ItemType),
}
