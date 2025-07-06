use demoinfocs_rs::{
    bitreader::BitReader,
    commands::CommandBuilder,
    proto::msgs2::{CnetMsgTick, NetMessages},
};
use prost::Message;
use std::io::Cursor;

fn read_bytes<R: std::io::Read>(reader: &mut BitReader<R>, len: usize) -> Vec<u8> {
    (0..len).map(|_| reader.read_int(8) as u8).collect()
}

#[test]
#[ignore]
fn encode_tick_message() {
    let mut builder = CommandBuilder::new();
    let msg = CnetMsgTick {
        tick: Some(5),
        ..Default::default()
    };
    builder
        .push_net_message(NetMessages::NetTick, &msg)
        .unwrap();
    let packet = builder.into_packet();
    let data = packet.data.unwrap();

    let mut reader = BitReader::new_large(Cursor::new(&data[..]));
    let msg_type = reader.read_ubit_int();
    assert_eq!(msg_type, NetMessages::NetTick as u32);
    reader.read_int(2); // padding bits
    let size = reader.read_varint32() as usize;
    let bytes = read_bytes(&mut reader, size);
    let decoded = CnetMsgTick::decode(&bytes[..]).unwrap();
    assert_eq!(decoded.tick, Some(5));
}

#[test]
fn raw_bytes_builder() {
    let msg = CnetMsgTick {
        tick: Some(7),
        ..Default::default()
    };
    let encoded = msg.encode_to_vec();

    let mut raw = CommandBuilder::new();
    raw.push_raw_net_message(NetMessages::NetTick, &encoded)
        .unwrap();
    let mut typed = CommandBuilder::new();
    typed.push_net_message(NetMessages::NetTick, &msg).unwrap();

    assert_eq!(raw.into_bytes(), typed.into_bytes());
}
