use idmangler_lib::{
    types::{Element, TransformVersion},
    DataDecoder, DataEncoder, PowderData,
};

#[test]
fn powderdata_roundtrip() {
    let mut out = Vec::new();
    let mut pow = Vec::new();

    for i in 0..51 {
        pow.push(Some((Element::Air, (i % 6) + 1)));
        pow.push(Some((Element::Earth, (i % 6) + 1)));
        pow.push(Some((Element::Fire, (i % 6) + 1)));
        pow.push(Some((Element::Thunder, (i % 6) + 1)));
        pow.push(Some((Element::Water, (i % 6) + 1)));
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
