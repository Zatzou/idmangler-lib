use idmangler_lib::encoding::string::{decode_char, decode_string, encode_string, BadCodepoint};

#[test]
fn roundtrip_allbytes() {
    let mut bytes = Vec::new();
    for i in 0..=u16::MAX {
        let [a, b] = i.to_ne_bytes();
        bytes.push(a);
        bytes.push(b);
    }

    let enc = encode_string(&bytes);
    let dec = decode_string(&enc).unwrap();

    assert_eq!(bytes, dec);
}

#[test]
fn bad_codepoints() {
    for point in ['a' as u32, 0x0EFFFF] {
        let res = decode_char(char::from_u32(point).unwrap());

        match res {
            Ok(_) => unreachable!(),
            Err(BadCodepoint(e)) => assert_eq!(point, e),
        }
    }
}
