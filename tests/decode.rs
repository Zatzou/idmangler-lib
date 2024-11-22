use idmangler_lib::{
    block::{
        anyblock::AnyBlockVec, EndData, IdentificationData, NameData, PowderData, RerollData,
        ShinyData, StartData, TypeData,
    },
    types::{Element, EncodingVersion, ItemType, Powder, RollType, Stat},
};

#[test]
fn simple_item() {
    let input = "󰀀󰄀󰉂󷉥󶕺󶕨󶅮󶑳󰀃󰀁󰉑󰨭󰋿";

    let mut decode = AnyBlockVec::decode(&input).unwrap();

    assert_eq!(
        decode.take::<StartData>(),
        Some(StartData(EncodingVersion::Version1))
    );

    assert_eq!(decode.take::<TypeData>(), Some(TypeData(ItemType::Gear)));

    assert_eq!(
        decode.take::<NameData>(),
        Some(NameData(String::from("Breezehands")))
    );

    assert_eq!(
        decode.take::<IdentificationData>(),
        Some(IdentificationData {
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
        })
    );

    assert_eq!(decode.take::<EndData>(), Some(EndData));

    assert!(decode.is_empty());
}

#[test]
fn complex_item() {
    let input = "󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿";

    let mut decode = AnyBlockVec::decode(&input).unwrap();

    assert_eq!(
        decode.take::<StartData>(),
        Some(StartData(EncodingVersion::Version1))
    );

    assert_eq!(decode.take::<TypeData>(), Some(TypeData(ItemType::Gear)));

    assert_eq!(
        decode.take::<NameData>(),
        Some(NameData(String::from("Immolation")))
    );

    assert_eq!(
        decode.take::<IdentificationData>(),
        Some(IdentificationData {
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
        })
    );

    assert_eq!(
        decode.take::<PowderData>(),
        Some(PowderData {
            powder_slots: 3,
            powders: vec![
                Powder::try_from((Element::Air, 6)).unwrap(),
                Powder::try_from((Element::Fire, 6)).unwrap(),
                Powder::try_from((Element::Fire, 6)).unwrap(),
            ],
        })
    );

    assert_eq!(decode.take::<RerollData>(), Some(RerollData(4)));

    assert_eq!(
        decode.take::<ShinyData>(),
        Some(ShinyData { id: 6, val: 0 })
    );

    assert_eq!(decode.take::<EndData>(), Some(EndData));

    assert!(decode.is_empty());
}

#[test]
fn negative_ids() {
    let input = "󰀀󰄀󰉇󶡯󷍴󶱹󲁃󶅰󰀃󰌁󰀣󰡽󳶂󰄬󲄋󷸄󰈀􏿮";

    let mut decode = AnyBlockVec::decode(&input).unwrap();

    assert_eq!(
        decode.take::<StartData>(),
        Some(StartData(EncodingVersion::Version1))
    );

    assert_eq!(decode.take::<TypeData>(), Some(TypeData(ItemType::Gear)));

    assert_eq!(
        decode.take::<NameData>(),
        Some(NameData(String::from("Ghostly Cap")))
    );

    assert_eq!(
        decode.take::<IdentificationData>(),
        Some(IdentificationData {
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
        })
    );

    assert_eq!(
        decode.take::<PowderData>(),
        Some(PowderData {
            powder_slots: 2,
            powders: Vec::new(),
        })
    );

    assert_eq!(decode.take::<EndData>(), Some(EndData));

    assert!(decode.is_empty());
}
