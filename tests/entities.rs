use cs_demo_parser::bitreader::BitReader;
use cs_demo_parser::sendtables::entity::{FlattenedPropEntry, PropertyValue};
use cs_demo_parser::sendtables::propdecoder::{
    PROP_TYPE_INT, SendPropertyFlags, SendTableProperty,
};
use cs_demo_parser::sendtables::serverclass::ServerClass;

#[test]
#[ignore]
fn test_new_entity_with_baseline() {
    let mut sc = ServerClass::default();
    sc.flattened_props = vec![FlattenedPropEntry {
        name: "prop".into(),
        prop: SendTableProperty {
            flags: SendPropertyFlags::empty(),
            low_value: 0.0,
            high_value: 0.0,
            number_of_bits: 8,
            number_of_elements: 0,
            priority: 0,
            raw_type: PROP_TYPE_INT,
        },
        array_element_prop: None,
    }];
    sc.preprocessed_baseline.insert(
        0,
        PropertyValue {
            int_val: 42,
            ..Default::default()
        },
    );
    let data = [0xffu8, 0xffu8];
    let mut reader = BitReader::new_small(&data[..]);
    let mut sc_clone = sc.clone();
    let ent = sc_clone.new_entity(&mut reader, 1, 1);
    assert_eq!(42, ent.props[0].value.int_val);
}
