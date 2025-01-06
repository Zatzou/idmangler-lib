//! This library provides an implementation of the wynntils idstring format. This format is used by the Wynntils mod to encode items for sharing them.
//!
//! For working with items the [`item`] module provides tools for decoding and encoding items from and to strings.
//!
//! Alternatively the [`block`] module provides tools for working with the blocks that make up the idstring format.
//!
//! Typically idstrings are represented using an encoded string using unicode private use area characters, however the contents of an idstring
//! may also be represented as bytes. Conversion between the two is provided by the [`encoding::string`] module.

pub mod block;
pub mod encoding;
pub mod item;
pub(crate) mod macros;
pub mod types;
