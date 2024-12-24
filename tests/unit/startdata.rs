use idmangler_lib::{
    block::{AnyBlock, StartData},
    decode_bytes,
    encoding::{DataEncoder, DecodeError},
    types::EncodingVersion,
};

#[test]
fn encode_startdata() {
    let mut out = Vec::new();
    let ver = EncodingVersion::Version1;

    StartData(ver).encode(ver, &mut out).unwrap();

    assert_eq!(out, Vec::from([0, 0]));
}

#[test]
fn decode_startdata() {
    let bytes: Vec<u8> = Vec::from([0, 0]);

    let ver = decode_bytes(&bytes).unwrap();
    let ver = ver.get(0).unwrap();

    assert_eq!(
        ver,
        &AnyBlock::StartData(StartData(EncodingVersion::Version1)),
        "Expected StartData(EncodingVersion::Version1), got {:?}",
        ver
    );
}

#[test]
fn decode_bad_startdata() {
    let bytes: Vec<u8> = Vec::from([0, 255]);

    let ver = decode_bytes(&bytes);

    match ver {
        Ok(_) => panic!("Expected an error"),
        Err(e) => match e.error {
            DecodeError::UnknownVersion(d) => assert_eq!(d.0, 255),
            _ => panic!("Expected an UnknownVersion error, got {:?}", e),
        },
    }
}
