use idmangler_lib::{
    encoding::{block::NameData, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::EncodingVersion,
};

#[test]
fn namedata_roundtrip() {
    let mut out = Vec::new();
    let namedata = NameData(String::from("test"));

    namedata
        .encode(EncodingVersion::Version1, &mut out)
        .unwrap();

    assert_eq!(out, Vec::from([2, b't', b'e', b's', b't', 0]));
}

#[test]
fn decode_bad_namedata() {
    let bytes: Vec<u8> = Vec::from([2, 255]);

    let ver = NameData::decode_data(&mut bytes.iter().copied(), EncodingVersion::Version1);

    match ver {
        Ok(_) => panic!("Expected an error"),
        Err(e) => match e {
            DecodeError::BadString => {}
            _ => panic!("Expected an BadString error, got {:?}", e),
        },
    }
}

#[test]
fn encode_bad_namedata() {
    for s in ["ö", "😀"] {
        let mut out = Vec::new();
        let namedata = NameData(String::from(s));

        let res = namedata.encode(EncodingVersion::Version1, &mut out);

        match res {
            Ok(_) => panic!("Expected an error"),
            Err(e) => match e {
                EncodeError::NonAsciiString => {}
                _ => panic!("Expected an BadString error, got {:?}", e),
            },
        }
    }
}
