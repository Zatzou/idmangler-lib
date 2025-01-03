use idmangler_lib::encoding::string::{decode_string, encode_string};

#[test]
fn roundtrip_allbytes() {
    let mut bytes = Vec::new();
    for i in 0..=255 {
        bytes.push(i);
    }

    let enc = encode_string(&bytes);
    let dec = decode_string(&enc).unwrap();

    assert_eq!(bytes, dec);
}
