use idmangler_lib::{
    encoding::{decode_string, encode_string},
    types::{ItemType, Powders, RollType, Stat, TransformVersion},
    DataEncoder, EndData, IdentificationData, NameData, PowderData, RerollData, ShinyData,
    StartData, TypeData,
};

#[test]
fn simple_item() {
    let mut out = Vec::new();
    let ver = TransformVersion::Version1;

    // start data
    StartData(ver).encode(ver, &mut out).unwrap();

    // type data
    TypeData(ItemType::Gear).encode(ver, &mut out).unwrap();

    // name data
    NameData(String::from("Breezehands"))
        .encode(ver, &mut out)
        .unwrap();

    // identifications
    IdentificationData {
        identifications: vec![
            Stat {
                kind: 81,
                base: Some(5),
                roll: RollType::PreIdentified,
            },
            Stat {
                kind: 45,
                base: Some(1),
                roll: RollType::PreIdentified,
            },
        ],
        extended_encoding: true,
    }
    .encode(ver, &mut out)
    .unwrap();

    // end data
    EndData.encode(ver, &mut out).unwrap();

    // do the final encode
    let outstr = encode_string(&out);

    // compare to an item encoded by wynntils
    assert_eq!(&outstr, "󰀀󰄀󰉂󷉥󶕺󶕨󶅮󶑳󰀃󰀁󰉑󰨭󰋿");
}

#[test]
fn complex_item() {
    let mut out = Vec::new();
    let ver = TransformVersion::Version1;

    // start data
    StartData(ver).encode(ver, &mut out).unwrap();

    // type data
    TypeData(ItemType::Gear).encode(ver, &mut out).unwrap();

    // name data
    NameData(String::from("Immolation"))
        .encode(ver, &mut out)
        .unwrap();

    // identifications
    IdentificationData {
        identifications: vec![
            Stat {
                kind: 24,
                base: None,
                roll: RollType::Value(81),
            },
            Stat {
                kind: 23,
                base: None,
                roll: RollType::Value(73),
            },
            Stat {
                kind: 18,
                base: None,
                roll: RollType::Value(75),
            },
            Stat {
                kind: 4,
                base: None,
                roll: RollType::Value(102),
            },
            Stat {
                kind: 2,
                base: None,
                roll: RollType::Value(48),
            },
        ],
        extended_encoding: false,
    }
    .encode(ver, &mut out)
    .unwrap();

    // powders
    PowderData {
        powder_slots: 3,
        powders: vec![
            Some((Powders::AIR, 6)),
            Some((Powders::FIRE, 6)),
            Some((Powders::FIRE, 6)),
        ],
    }
    .encode(ver, &mut out)
    .unwrap();

    // Rerolls
    RerollData(4).encode(ver, &mut out).unwrap();

    // Shiny
    ShinyData { id: 6, val: 0 }.encode(ver, &mut out).unwrap();

    // end data
    EndData.encode(ver, &mut out).unwrap();

    let outstr = encode_string(&out);

    assert_eq!(out, decode_string("󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿"));
    assert_eq!(&outstr, "󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿");
}

#[test]
fn negative_ids() {
    let mut out = Vec::new();
    let ver = TransformVersion::Version1;

    // start data
    StartData(ver).encode(ver, &mut out).unwrap();

    // type data
    TypeData(ItemType::Gear).encode(ver, &mut out).unwrap();

    // name data
    NameData(String::from("Ghostly Cap"))
        .encode(ver, &mut out)
        .unwrap();

    // identifications
    IdentificationData {
        identifications: vec![
            Stat {
                kind: 35,
                base: Some(4),
                roll: RollType::Value(125),
            },
            Stat {
                kind: 61,
                base: Some(65),
                roll: RollType::Value(44),
            },
            Stat {
                kind: 33,
                base: Some(-6),
                roll: RollType::Value(126),
            },
        ],
        extended_encoding: true,
    }
    .encode(ver, &mut out)
    .unwrap();

    // Wynntils encodes an empty powderdata
    PowderData {
        powder_slots: 2,
        powders: Vec::new(),
    }
    .encode(ver, &mut out)
    .unwrap();

    // end data
    EndData.encode(ver, &mut out).unwrap();

    let outstr = encode_string(&out);

    assert_eq!(&outstr, "󰀀󰄀󰉇󶡯󷍴󶱹󲁃󶅰󰀃󰌁󰀣󰡽󳶂󰄬󲄋󷸄󰈀􏿮");
}
