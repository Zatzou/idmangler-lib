use idmangler_lib::{
    block::IdentificationData,
    encoding::{DataDecoder, DataEncoder},
    types::{EncodingVersion, RollType, Stat},
};

#[test]
fn identdata_roundtrip() {
    for extended in [true, false] {
        let mut out = Vec::new();
        let mut idents = Vec::new();

        for i in 0..255 {
            idents.push(Stat {
                kind: i,
                base: if extended { Some(100) } else { None },
                roll: RollType::Value(i),
            });
        }

        let identdata = IdentificationData {
            extended_encoding: extended,
            identifications: idents,
        };

        identdata
            .encode(EncodingVersion::V1, &mut out)
            .unwrap();

        let mut iter = out.iter().copied().skip(1);
        let decoded =
            IdentificationData::decode_data(&mut iter, EncodingVersion::V1).unwrap();

        assert_eq!(identdata, decoded);
    }
}

#[test]
fn identdata_noencode() {
    let identdata = IdentificationData {
        extended_encoding: false,
        identifications: vec![Stat {
            kind: 0,
            base: None,
            roll: RollType::PreIdentified,
        }],
    };

    assert!(
        !identdata.should_encode_data(EncodingVersion::V1)
    );
}

#[test]
fn bad_identdata() {
    // first byte is the number of identifications, but array contains less than 100 so this should fail
    let mut iter = [100, 0, 0, 0].iter().copied();
    assert!(IdentificationData::decode_data(&mut iter, EncodingVersion::V1).is_err());

    // extended coding always requires base to be present
    let mut out = Vec::new();
    let identdata = IdentificationData {
        extended_encoding: true,
        identifications: vec![Stat {
            kind: 0,
            base: None,
            roll: RollType::PreIdentified,
        }],
    };

    assert!(identdata
        .encode(EncodingVersion::V1, &mut out)
        .is_err());
}
