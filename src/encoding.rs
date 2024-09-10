//! Encoding utilities provided by this library
//!
//! This module provides the encoding and decoding functions for the private-use area encoding used by wynntils.

use crate::data_transformer::DecodeError;

/// Encode bytes into a string using the wynntils byte encoding scheme
///
/// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/utils/EncodedByteBuffer.java#L87>
pub fn encode_string(data: &[u8]) -> String {
    let mut out = String::new();

    // unwraps here are safe as all the encoded characters can only fall within the private use areas used for the encoding
    for d in data.chunks(2) {
        if d.len() == 2 {
            if d[0] == 255 && d[1] >= 254 {
                out.push(char::from_u32(0x100000 + ((d[1] - 254) as u32)).unwrap());
            } else {
                out.push(char::from_u32(0xF0000 + ((d[0] as u32) << 8) + d[1] as u32).unwrap());
            }
        } else {
            // encode leftover singular bits with the seperate encoding
            out.push(char::from_u32(0x100000 + ((d[0] as u32) << 8) + 0xEE).unwrap());
        }
    }

    out
}

/// Decodes the bytes of a wynntils private area encoded string
///
/// This function does not check whether or not the encoded data is valid
///
/// <https://github.com/Wynntils/Wynntils/blob/main/common/src/main/java/com/wynntils/utils/EncodedByteBuffer.java#L33>
pub fn decode_string(data: &str) -> Vec<u8> {
    let mut out = Vec::new();

    for c in data.chars() {
        let n: u32 = c.into();

        // special case Private use area B
        if n > 0x100000 {
            // single byte
            if n & 0xFF == 0xEE {
                out.push(((n & 0xFF00) >> 8) as u8);

                assert!(((n & 0xFF00) >> 8) <= 255, "Invalid codepoint: {n:06X}");
                continue;
            }

            // two bytes

            out.push(255);
            out.push((254 + (n & 0xFF)) as u8);

            // Only 0x100000-0x100001 are used
            assert!(n < 0x100002, "Invalid codepoint: {n:06X}");
            continue;
        }

        out.push(((n & 0xFF00) >> 8) as u8);
        out.push((n & 0x00FF) as u8);
    }

    out
}

/// Encode an integer of variable size (up to i64) into bytes using the format which wynntils uses
pub(crate) fn encode_varint(value: i64) -> Vec<u8> {
    // zigzag encoding magic
    // removes sign bit so values are only positive
    let value = ((value << 1) ^ (value >> 63)) as u64;

    // 7 bits per byte
    // highest bit is used to indicate end of encoding

    // calulate number of bytes needed
    let mut numofbytes = 1;
    let mut temp = value >> 7;
    while temp != 0 {
        numofbytes += 1;
        temp >>= 7;
    }

    let mut outbytes = Vec::new();
    for i in 0..numofbytes {
        let mut next = (value >> (7 * i)) as u8 & 0x7F;

        // indicate that we are **not** done by setting the highest bit
        if i < numofbytes - 1 {
            next |= 0b10000000;
        }

        outbytes.push(next);
    }

    outbytes
}

/// Decode a variable sized integer (max i64) from the identification data bytestream
pub(crate) fn decode_varint(bytes: &mut impl Iterator<Item = u8>) -> Result<i64, DecodeError> {
    let mut value = 0;

    let mut data = Vec::new();
    loop {
        let b = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

        data.push(b);

        if (b & 0b10000000) == 0 {
            break;
        }
    }

    for (i, n) in data.into_iter().enumerate() {
        value |= ((n & 0b01111111) as i64) << (7 * i);
    }

    Ok((value >> 1) ^ -(value & 1))
}
