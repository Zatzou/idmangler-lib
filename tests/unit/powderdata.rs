use idmangler_lib::{
    types::{Powders, TransformVersion},
    DataDecoder, DataEncoder, PowderData,
};

#[test]
fn powderdata_roundtrip() {
    let mut out = Vec::new();
    let mut pow = Vec::new();

    for _ in 0..51 {
        pow.push((Powders::AIR, 0));
        pow.push((Powders::EARTH, 0));
        pow.push((Powders::FIRE, 0));
        pow.push((Powders::THUNDER, 0));
        pow.push((Powders::WATER, 0));
    }

    let powders = PowderData {
        powder_slots: 123,
        powders: pow,
    };

    powders
        .encode(TransformVersion::Version1, &mut out)
        .unwrap();

    let mut iter = out.iter().copied().skip(1);
    let decoded = PowderData::decode_data(&mut iter, TransformVersion::Version1).unwrap();

    assert_eq!(powders, decoded);
}
