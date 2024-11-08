use idmangler_lib::{
    encoding::{block::TypeData, DataDecoder, DataEncoder, DecodeError},
    types::{EncodingVersion, ItemType},
};

#[test]
fn typedata_roundtrip() {
    for t in [
        ItemType::Gear,
        ItemType::Tome,
        ItemType::Charm,
        ItemType::CraftedGear,
        ItemType::CraftedConsu,
    ] {
        let td = TypeData(t);
        let mut buf = Vec::new();

        // encode the data
        td.encode(EncodingVersion::Version1, &mut buf).unwrap();

        // decode the data
        let mut iter = buf.iter().copied().skip(1); // skip the transformer id as we are not doing full decodes
        let td2 = TypeData::decode_data(&mut iter, EncodingVersion::Version1).unwrap();

        // check the results
        assert_eq!(td, td2);
        assert_eq!(buf, vec![1, td.0.into()]);
    }
}

#[test]
fn decode_bad_typedata() {
    let bytes: Vec<u8> = Vec::from([255]);

    let ver = TypeData::decode_data(&mut bytes.iter().copied(), EncodingVersion::Version1);

    match ver {
        Ok(_) => panic!("Expected an error"),
        Err(e) => match e {
            DecodeError::BadItemType(d) => assert_eq!(d, 255),
            _ => panic!("Expected an BadItemType error, got {:?}", e),
        },
    }
}
