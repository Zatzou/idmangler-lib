use idmangler_lib::{
    decode,
    encoding::block::{
        EndData, IdentificationData, NameData, PowderData, RerollData, ShinyData, StartData,
        TypeData,
    },
    types::{Element, EncodingVersion, ItemType, RollType, Stat},
};

#[test]
fn simple_item() {
    let input = "󰀀󰄀󰉂󷉥󶕺󶕨󶅮󶑳󰀃󰀁󰉑󰨭󰋿";

    let decode = decode(&input).unwrap();

    assert_eq!(
        decode,
        vec![
            StartData(EncodingVersion::Version1).into(),
            TypeData(ItemType::Gear).into(),
            NameData(String::from("Breezehands")).into(),
            IdentificationData {
                identifications: vec![
                    Stat {
                        kind: 81,
                        base: Some(5),
                        roll: RollType::PreIdentified
                    },
                    Stat {
                        kind: 45,
                        base: Some(1),
                        roll: RollType::PreIdentified
                    }
                ],
                extended_encoding: true,
            }
            .into(),
            EndData.into()
        ]
    );
}

#[test]
fn complex_item() {
    let input = "󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿";

    let decode = decode(&input).unwrap();

    assert_eq!(
        decode,
        vec![
            StartData(EncodingVersion::Version1).into(),
            TypeData(ItemType::Gear).into(),
            NameData(String::from("Immolation")).into(),
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
            .into(),
            PowderData {
                powder_slots: 3,
                powders: vec![
                    Some((Element::Air, 6)),
                    Some((Element::Fire, 6)),
                    Some((Element::Fire, 6))
                ],
            }
            .into(),
            RerollData(4).into(),
            ShinyData { id: 6, val: 0 }.into(),
            EndData.into()
        ]
    );
}

#[test]
fn negative_ids() {
    let input = "󰀀󰄀󰉇󶡯󷍴󶱹󲁃󶅰󰀃󰌁󰀣󰡽󳶂󰄬󲄋󷸄󰈀􏿮";

    let decode = decode(&input).unwrap();

    assert_eq!(
        decode,
        vec![
            StartData(EncodingVersion::Version1).into(),
            TypeData(ItemType::Gear).into(),
            NameData(String::from("Ghostly Cap")).into(),
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
            .into(),
            PowderData {
                powder_slots: 2,
                powders: Vec::new(),
            }
            .into(),
            EndData.into()
        ]
    );
}
