use super::DecodeError;

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

    Ok((value as u64 >> 1) as i64 ^ -(value & 1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn roundtrip_varints() {
        for i in [
            0,
            1,
            -1,
            42,
            -42,
            1000,
            -1000,
            i16::MAX as i64,
            i16::MIN as i64,
            i32::MAX as i64,
            i32::MIN as i64,
            i64::MAX,
            i64::MIN,
        ] {
            let bytes = encode_varint(i);
            let n = decode_varint(&mut bytes.into_iter()).unwrap();

            assert_eq!(i, n);
        }
    }
}
