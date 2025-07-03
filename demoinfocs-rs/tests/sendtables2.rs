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
