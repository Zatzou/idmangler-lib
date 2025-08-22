use idmangler_lib::{
    block::PowderData,
    encoding::{DataDecoder, DataEncoder},
    types::{Element, EncodingVersion, Powder},
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
        powders: pow
            .into_iter()
            .map(|e| Powder::try_from(e).unwrap())
            .collect(),
    };

    powders.encode(EncodingVersion::V1, &mut out).unwrap();

    let mut iter = out.iter().copied().skip(1);
    let decoded = PowderData::decode_data(&mut iter, EncodingVersion::V1).unwrap();

    assert_eq!(powders, decoded);
}
