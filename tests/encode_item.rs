use idmangler_lib::{
    block::{IdentificationData, PowderData, RerollData, ShinyData},
    item::GenericItem,
    types::{Element, EncodingVersion, ItemType, Powder, RollType, Stat},
};

#[test]
fn simple_item() {
    let ver = EncodingVersion::Version1;

    let out = GenericItem {
        kind: ItemType::Gear,
        name: Some(String::from("Breezehands")),
        identifications: Some(IdentificationData {
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
        }),
        ..Default::default()
    }
    .encode(ver)
    .unwrap();

    // compare to an item encoded by wynntils
    assert_eq!(&out, "󰀀󰄀󰉂󷉥󶕺󶕨󶅮󶑳󰀃󰀁󰉑󰨭󰋿");
}

#[test]
fn complex_item() {
    let ver = EncodingVersion::Version1;

    let out = GenericItem {
        kind: ItemType::Gear,
        name: Some(String::from("Immolation")),
        identifications: Some(IdentificationData {
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
        }),
        powders: Some(PowderData {
            powder_slots: 3,
            powders: vec![
                Powder::try_from((Element::Air, 6)).unwrap(),
                Powder::try_from((Element::Fire, 6)).unwrap(),
                Powder::try_from((Element::Fire, 6)).unwrap(),
            ],
        }),
        rerolls: Some(RerollData(4)),
        shiny: Some(ShinyData { id: 6, val: 0 }),
        ..Default::default()
    }
    .encode(ver)
    .unwrap();

    assert_eq!(&out, "󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿");
}

#[test]
fn negative_ids() {
    let ver = EncodingVersion::Version1;

    let out = GenericItem {
        kind: ItemType::Gear,
        name: Some(String::from("Ghostly Cap")),
        identifications: Some(IdentificationData {
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
        }),
        powders: Some(PowderData {
            powder_slots: 2,
            powders: Vec::new(),
        }),
        ..Default::default()
    }
    .encode(ver)
    .unwrap();

    assert_eq!(&out, "󰀀󰄀󰉇󶡯󷍴󶱹󲁃󶅰󰀃󰌁󰀣󰡽󳶂󰄬󲄋󷸄󰈀􏿮");
}
