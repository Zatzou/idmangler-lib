//! Encoding and decoding between the private area encoding and bytes

use thiserror::Error;

/// Start of the supplementary private use area A
const AREA_A: u32 = 0x0F0000;
/// Start of the supplementary private use area B
const AREA_B: u32 = 0x100000;

/// Encode bytes into a string using the wynntils byte encoding scheme
///
/// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/utils/EncodedByteBuffer.java#L87>
pub fn encode_string(data: &[u8]) -> String {
    let mut out = String::with_capacity((data.len() + data.len() % 2) * 2);

    for d in data.chunks(2) {
        match *d {
            [a, b] => out.push(encode_char((a, Some(b)))),
            [a] => out.push(encode_char((a, None))),
            _ => unreachable!(),
        }
    }

    out
}

/// Encode a single char using the wynntils private use area encoding scheme
pub const fn encode_char(data: (u8, Option<u8>)) -> char {
    // unwraps here are safe as all the encoded characters can only fall within the private use areas used for the encoding
    match data {
        (0xFF, Some(b)) if b >= 254 => char::from_u32(AREA_B + ((b - 254) as u32)).unwrap(),
        (a, Some(b)) => char::from_u32(AREA_A + ((a as u32) << 8) + b as u32).unwrap(),
        (a, None) => char::from_u32(AREA_B + ((a as u32) << 8) + 0xEE).unwrap(),
    }
}

/// An invalid codepoint was encountered during decoding
#[derive(Error, Debug)]
#[error("Invalid codepoint: {0:06X}")]
pub struct BadCodepoint(pub u32);

/// Decodes the bytes of a wynntils private area encoded string
///
/// This function does not check whether or not the encoded data is valid
///
/// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/utils/EncodedByteBuffer.java#L33>
pub fn decode_string(data: &str) -> Result<Vec<u8>, BadCodepoint> {
    decode_chars(data.chars())
}

/// Decodes the bytes of a wynntils private area encoded string from an iterator of chars
pub fn decode_chars(data: impl Iterator<Item = char>) -> Result<Vec<u8>, BadCodepoint> {
    let mut out = Vec::with_capacity(data.size_hint().0 * 2);

    for c in data {
        out.extend(decode_char(c)?);
    }

    Ok(out)
}

/// Type representing the output of a single char decode operation. This type is an iterator over 1 or 2 bytes. The None variant is used to signal that the iterator is empty.
#[derive(Debug, Clone, Copy)] // TODO: Remove Copy in the next breaking release
pub enum OutputByte {
    /// Only used internally to signal that the iterator is empty
    None,
    /// Output is a single byte
    One(u8),
    /// Output is two bytes
    Two(u8, u8),
}

impl Iterator for OutputByte {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Self::None => None,
            Self::One(x) => {
                *self = Self::None;
                Some(x)
            }
            Self::Two(a, b) => {
                *self = Self::One(b);
                Some(a)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match *self {
            Self::None => (0, Some(0)),
            Self::One(_) => (1, Some(1)),
            Self::Two(_, _) => (2, Some(2)),
        }
    }
}

/// Decode a single char from the wynntils private use area encoding scheme. This function returns 1 or 2 bytes or an error if the codepoint is not valid within the encoding scheme.
pub fn decode_char(data: char) -> Result<OutputByte, BadCodepoint> {
    use OutputByte::{One, Two};
    let n = u32::from(data);

    if !(AREA_A..=(AREA_B + 0xFFFF)).contains(&n) {
        return Err(BadCodepoint(n));
    }

    // special cases
    if n >= AREA_B {
        // single byte
        if n & 0xFF == 0xEE {
            return Ok(One(((n & 0xFF00) >> 8) as u8));
        }

        // two bytes
        return Ok(Two(255, (254 + (n & 0xFF)) as u8));
    }

    // normal case
    Ok(Two(((n & 0xFF00) >> 8) as u8, (n & 0x00FF) as u8))
}
