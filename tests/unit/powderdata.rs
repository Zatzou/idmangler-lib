use idmangler_lib::{
    block::PowderData,
    encoding::{DataDecoder, DataEncoder},
    types::{Element, EncodingVersion},
};

#[test]
fn powderdata_roundtrip() {
    let mut out = Vec::new();
    let mut pow = Vec::new();

    for i in 0..51 {
        pow.push((Element::Air, (i % 6) + 1));
        pow.push((Element::Earth, (i % 6) + 1));
        pow.push((Element::Fire, (i % 6) + 1));
        pow.push((Element::Thunder, (i % 6) + 1));
        pow.push((Element::Water, (i % 6) + 1));
    }

    let powders = PowderData {
        powder_slots: 123,
        powders: pow,
    };

    powders.encode(EncodingVersion::Version1, &mut out).unwrap();

    let mut iter = out.iter().copied().skip(1);
    let decoded = PowderData::decode_data(&mut iter, EncodingVersion::Version1).unwrap();

    assert_eq!(powders, decoded);
}
