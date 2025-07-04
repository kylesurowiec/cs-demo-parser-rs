use demoinfocs_rs::bitreader::BitReader;
use demoinfocs_rs::sendtables::entity::{Entity, FlattenedPropEntry, Property, PropertyValue};
use demoinfocs_rs::sendtables::propdecoder::{
    PROP_TYPE_INT, PROP_TYPE_INT64, PropertyDecoder, SendPropertyFlags, SendTableProperty,
};
use demoinfocs_rs::sendtables::serverclass::ServerClass;
use std::sync::Arc;

#[test]
fn test_server_class_getters() {
    let sc = ServerClass {
        id: 1,
        name: "TestClass".into(),
        data_table_id: 2,
        data_table_name: "ADataTable".into(),
        ..Default::default()
    };
    assert_eq!(1, sc.id());
    assert_eq!("TestClass", sc.name());
    assert_eq!(2, sc.data_table_id());
    assert_eq!("ADataTable", sc.data_table_name());
}

#[test]
fn test_server_class_property_entries() {
    let mut sc = ServerClass::default();
    assert!(sc.property_entries().is_empty());
    sc.flattened_props = vec![
        FlattenedPropEntry {
            name: "prop1".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: PROP_TYPE_INT,
            },
            array_element_prop: None,
        },
        FlattenedPropEntry {
            name: "prop2".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: PROP_TYPE_INT,
            },
            array_element_prop: None,
        },
    ];
    let entries = sc.property_entries();
    assert_eq!(entries, vec!["prop1".to_string(), "prop2".to_string()]);
}

#[test]
fn test_server_class_string() {
    let mut sc = ServerClass {
        id: 1,
        name: "TestClass".into(),
        data_table_id: 2,
        data_table_name: "ADataTable".into(),
        ..Default::default()
    };
    let expected = "serverClass: id=1 \
                    name=TestClass\n\tdataTableId=2\n\tdataTableName=ADataTable\n\tbaseClasses:\n\\
                    t\t-\n\tproperties:\n\t\t-";
    assert_eq!(expected, sc.to_string());
    sc.base_classes = vec![
        ServerClass {
            name: "AnotherClass".into(),
            ..Default::default()
        },
        ServerClass {
            name: "YetAnotherClass".into(),
            ..Default::default()
        },
    ];
    sc.flattened_props = vec![
        FlattenedPropEntry {
            name: "prop1".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: PROP_TYPE_INT,
            },
            array_element_prop: None,
        },
        FlattenedPropEntry {
            name: "prop2".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: PROP_TYPE_INT,
            },
            array_element_prop: None,
        },
    ];
    let expected2 = "serverClass: id=1 \
                     name=TestClass\n\tdataTableId=2\n\tdataTableName=ADataTable\n\tbaseClasses:\\
                     n\t\tAnotherClass\n\t\tYetAnotherClass\n\tproperties:\n\t\tprop1\n\t\tprop2";
    assert_eq!(expected2, sc.to_string());
}

#[test]
fn test_entity_properties() {
    let ent = Entity {
        server_class: Arc::new(ServerClass::default()),
        id: 0,
        serial_num: 0,
        props: vec![Property {
            entry: FlattenedPropEntry {
                name: "test".into(),
                prop: SendTableProperty {
                    flags: SendPropertyFlags::empty(),
                    low_value: 0.0,
                    high_value: 0.0,
                    number_of_bits: 0,
                    number_of_elements: 0,
                    priority: 0,
                    raw_type: PROP_TYPE_INT,
                },
                array_element_prop: None,
            },
            value: PropertyValue {
                int_val: 1,
                ..Default::default()
            },
        }],
    };
    assert_eq!(ent.properties()[0].name(), "test");
}

#[test]
fn test_entity_property_value() {
    let sc = Arc::new(ServerClass {
        flattened_props: vec![
            FlattenedPropEntry {
                name: "myProp".into(),
                prop: SendTableProperty {
                    flags: SendPropertyFlags::empty(),
                    low_value: 0.0,
                    high_value: 0.0,
                    number_of_bits: 0,
                    number_of_elements: 0,
                    priority: 0,
                    raw_type: PROP_TYPE_INT,
                },
                array_element_prop: None,
            },
            FlattenedPropEntry {
                name: "test".into(),
                prop: SendTableProperty {
                    flags: SendPropertyFlags::empty(),
                    low_value: 0.0,
                    high_value: 0.0,
                    number_of_bits: 0,
                    number_of_elements: 0,
                    priority: 0,
                    raw_type: PROP_TYPE_INT,
                },
                array_element_prop: None,
            },
            FlattenedPropEntry {
                name: "anotherOne".into(),
                prop: SendTableProperty {
                    flags: SendPropertyFlags::empty(),
                    low_value: 0.0,
                    high_value: 0.0,
                    number_of_bits: 0,
                    number_of_elements: 0,
                    priority: 0,
                    raw_type: PROP_TYPE_INT,
                },
                array_element_prop: None,
            },
        ],
        ..Default::default()
    });
    let ent = Entity {
        server_class: sc.clone(),
        id: 1,
        serial_num: 1337,
        props: vec![
            Property {
                entry: sc.flattened_props[0].clone(),
                value: PropertyValue {
                    int_val: 10,
                    ..Default::default()
                },
            },
            Property {
                entry: sc.flattened_props[1].clone(),
                value: PropertyValue {
                    int_val: 20,
                    ..Default::default()
                },
            },
            Property {
                entry: sc.flattened_props[2].clone(),
                value: PropertyValue {
                    int_val: 30,
                    ..Default::default()
                },
            },
        ],
    };
    assert_eq!(
        Some(PropertyValue {
            int_val: 20,
            ..Default::default()
        }),
        ent.property_value("test")
    );
    assert!(ent.property_public("not_found").is_none());
}

#[test]
fn test_property_value_bool() {
    let val = PropertyValue {
        int_val: 1,
        ..Default::default()
    };
    assert!(val.bool_val());
}

#[test]
fn test_decode_int64() {
    // prepare bytes
    let expected: i64 = 76561198000697560;
    let mut b = Vec::new();
    b.extend_from_slice(&expected.to_le_bytes());
    let mut reader = BitReader::new_small(&b[..]);
    let mut prop = Property {
        entry: FlattenedPropEntry {
            name: "".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::UNSIGNED,
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 64,
                number_of_elements: 0,
                priority: 0,
                raw_type: PROP_TYPE_INT64,
            },
            array_element_prop: None,
        },
        value: PropertyValue::default(),
    };
    PropertyDecoder.decode_prop(&mut prop, &mut reader);
    assert_eq!(expected, prop.value.int64_val);
}

#[test]
#[should_panic]
fn test_decode_prop_unknown() {
    let mut prop = Property {
        entry: FlattenedPropEntry {
            name: "".into(),
            prop: SendTableProperty {
                flags: SendPropertyFlags::empty(),
                low_value: 0.0,
                high_value: 0.0,
                number_of_bits: 0,
                number_of_elements: 0,
                priority: 0,
                raw_type: -1,
            },
            array_element_prop: None,
        },
        value: PropertyValue::default(),
    };
    let mut reader = BitReader::new_small(&[][..]);
    PropertyDecoder.decode_prop(&mut prop, &mut reader);
}
