use demoinfocs_rs::proto::msg::csvc_msg_class_info::ClassT;
use demoinfocs_rs::proto::msg::{CsvcMsgClassInfo, CsvcMsgPacketEntities};
use demoinfocs_rs::sendtables2::Parser;
use demoinfocs_rs::sendtables2::proto::{
    CsvcMsgFlattenedSerializer, ProtoFlattenedSerializerFieldT, ProtoFlattenedSerializerT,
};
use prost::Message;

fn encode_var(mut value: u32) -> Vec<u8> {
    let mut out = Vec::new();
    while value >= 0x80 {
        out.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
    out
}

struct BitWriter {
    buf: Vec<u8>,
    bit_val: u64,
    bit_count: u32,
}

impl BitWriter {
    fn new() -> Self {
        Self {
            buf: Vec::new(),
            bit_val: 0,
            bit_count: 0,
        }
    }

    fn write_bits(&mut self, mut value: u32, mut n: u32) {
        while n > 0 {
            let take = (8 - self.bit_count).min(n);
            let mask = (1u32 << take) - 1;
            self.bit_val |= ((value & mask) as u64) << self.bit_count;
            self.bit_count += take;
            value >>= take;
            n -= take;
            if self.bit_count == 8 {
                self.buf.push(self.bit_val as u8);
                self.bit_val = 0;
                self.bit_count = 0;
            }
        }
    }

    fn write_byte(&mut self, b: u8) {
        self.write_bits(b as u32, 8);
    }

    fn write_var(&mut self, mut value: u32) {
        while value >= 0x80 {
            self.write_byte(((value & 0x7f) as u8) | 0x80);
            value >>= 7;
        }
        self.write_byte(value as u8);
    }

    fn write_ubit_var(&mut self, value: u32) {
        // only supports small values for tests
        self.write_bits(value, 6);
    }

    fn into_bytes(mut self) -> Vec<u8> {
        if self.bit_count > 0 {
            self.buf.push(self.bit_val as u8);
        }
        self.buf
    }
}

#[test]
fn test_on_server_info() {
    let mut p = Parser::new();
    let msg = demoinfocs_rs::proto::msg::CsvcMsgServerInfo {
        max_classes: Some(255),
        ..Default::default()
    };
    p.on_server_info(&msg);
    assert!(p.class_id_size() > 0);
}

#[test]
fn test_parse_flattened_serializer() {
    let mut p = Parser::new();
    let msg = CsvcMsgFlattenedSerializer {
        serializers: vec![ProtoFlattenedSerializerT {
            serializer_name_sym: Some(1),
            serializer_version: Some(0),
            fields_index: vec![0],
        }],
        symbols: vec![
            "dummy".into(),
            "Test".into(),
            "int32".into(),
            "m_int".into(),
        ],
        fields: vec![ProtoFlattenedSerializerFieldT {
            var_type_sym: Some(2),
            var_name_sym: Some(3),
            bit_count: Some(8),
            low_value: None,
            high_value: None,
            encode_flags: None,
            field_serializer_name_sym: None,
            field_serializer_version: None,
            send_node_sym: None,
            var_encoder_sym: None,
            var_serializer_sym: None,
        }],
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let mut data = encode_var(buf.len() as u32);
    data.extend(buf);
    p.parse_packet(&data).unwrap();
    assert!(p.serializer("Test").is_some());
}

#[test]
fn test_class_info_and_entities() {
    let mut p = Parser::new();
    p.on_server_info(&demoinfocs_rs::proto::msg::CsvcMsgServerInfo {
        max_classes: Some(1),
        ..Default::default()
    });

    // serializer
    let msg = CsvcMsgFlattenedSerializer {
        serializers: vec![ProtoFlattenedSerializerT {
            serializer_name_sym: Some(1),
            serializer_version: Some(0),
            fields_index: vec![],
        }],
        symbols: vec!["dummy".into(), "Test".into()],
        fields: vec![],
    };
    let mut buf = Vec::new();
    msg.encode(&mut buf).unwrap();
    let mut data = encode_var(buf.len() as u32);
    data.extend(buf);
    p.parse_packet(&data).unwrap();

    // class info referencing serializer
    let class_msg = CsvcMsgClassInfo {
        create_on_client: Some(false),
        classes: vec![ClassT {
            class_id: Some(0),
            data_table_name: None,
            class_name: Some("Test".into()),
        }],
    };
    p.on_class_info(&class_msg);
    assert!(p.entity(0).is_none());

    // baseline (unused in this test but call for coverage)
    p.set_instance_baseline(0, vec![]);

    // craft packet entities creating entity 0
    let mut w = BitWriter::new();
    w.write_ubit_var(0); // index diff
    w.write_bits(2, 2); // enter pvs
    w.write_bits(0, p.class_id_size()); // class id
    w.write_bits(1, 17); // serial
    w.write_var(0); // length
    let data = w.into_bytes();
    let pe_msg = CsvcMsgPacketEntities {
        max_entries: Some(1),
        updated_entries: Some(1),
        is_delta: Some(false),
        update_baseline: Some(false),
        baseline: Some(0),
        delta_from: Some(0),
        entity_data: Some(data),
    };
    let created = p.parse_packet_entities(&pe_msg);
    assert_eq!(created.len(), 1);
    assert!(p.entity(0).is_some());
}
