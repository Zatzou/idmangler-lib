use idmangler_lib::{
    block::{IdentificationData, PowderData, RerollData, ShinyData},
    item::GenericItem,
    types::{Element, ItemType, Powder, RollType, Stat},
};

#[test]
fn simple_item() {
    let input = "󰀀󰄀󰉂󷉥󶕺󶕨󶅮󶑳󰀃󰀁󰉑󰨭󰋿";

    let decode = GenericItem::decode_string(input).unwrap();

    assert_eq!(
        decode,
        GenericItem {
            kind: ItemType::Gear,
            name: Some(String::from("Breezehands")),
            identifications: Some(IdentificationData {
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
            }),
            ..Default::default()
        }
    )
}

#[test]
fn complex_item() {
    let input = "󰀀󰄀󰉉󶵭󶽬󶅴󶥯󶸀󰌅󰀘󵄗󴤒󴬄󶘂󳀄󰌃󿘰󰔄󰘆󰃿";

    let decode = GenericItem::decode_string(input).unwrap();

    assert_eq!(
        decode,
        GenericItem {
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
            shiny: Some(ShinyData { id: 6, val: 0, rr: 0 }),
            ..Default::default()
        }
    );
}

#[test]
fn negative_ids() {
    let input = "󰀀󰄀󰉇󶡯󷍴󶱹󲁃󶅰󰀃󰌁󰀣󰡽󳶂󰄬󲄋󷸄󰈀􏿮";

    let decode = GenericItem::decode_string(input).unwrap();

    assert_eq!(
        decode,
        GenericItem {
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
    );
}

#[test]
fn v2_shiny() {
    let input = "󰀁󰄀󰉗󶅲󷀀󰌉󰄁󲤲󴖴󰅱󱅤󶔢󵥣󱢏󰍍󱦯󰥋󱜻󷀄󱹵󵇨󰉐󲛖󰑙󰐃󰀅󰔆󰘂󰃿";
    let decode = GenericItem::decode_string(input).unwrap();
    assert_eq!(
        decode,
        GenericItem { 
            kind: ItemType::Gear, 
            name: Some("Warp".to_owned()), 
            powders: Some(PowderData { 
                powder_slots: 3, powders: [].to_vec() 
            }), 
            identifications: Some(IdentificationData { 
                identifications: vec![
                    Stat { kind: 41, base: Some(25), roll: RollType::PreIdentified }, 
                    Stat { kind: 69, base: Some(90), roll: RollType::Value(113) }, 
                    Stat { kind: 17, base: Some(50), roll: RollType::Value(101) }, 
                    Stat { kind: 34, base: Some(-45), roll: RollType::Value(99) }, 
                    Stat { kind: 24, base: Some(-200), roll: RollType::Value(77) }, 
                    Stat { kind: 25, base: Some(-600), roll: RollType::Value(75) }, 
                    Stat { kind: 23, base: Some(-30), roll: RollType::Value(112) }, 
                    Stat { kind: 4, base: Some(15), roll: RollType::Value(117) }, 
                    Stat { kind: 81, base: Some(180), roll: RollType::Value(80) }, 
                    Stat { kind: 38, base: Some(299), roll: RollType::Value(89) }
                ], 
                extended_encoding: true }
            ), 
            rerolls: Some(RerollData(5)), 
            shiny: Some(ShinyData { id: 6, rr: 2, val: 0 }), 
            ..Default::default()
        }
    )
}