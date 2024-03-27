use iso15118::prelude::*;
use crate::mock_exi::*;


#[test]
fn app_hand_decode_everest() {
    // extract from Everest test suite
    let exi_data = [
        0x01, 0xfe, 0x80, 0x01, 0x00, 0x00, 0x00, 0x44, // vg2tp header
        0x80, 0x00, 0xf3, 0xab, 0x93, 0x71, 0xd3, 0x4b, 0x9b, 0x79, 0xd3, 0x9b, 0xa3, 0x21, 0xd3,
        0x4b, 0x9b, 0x79, 0xd1, 0x89, 0xa9, 0x89, 0x89, 0xc1, 0xd1, 0x69, 0x91, 0x81, 0xd2, 0x0a,
        0x18, 0x01, 0x00, 0x00, 0x04, 0x00, 0x40,
    ];

    let stream_exi = mock_network_input(&exi_data);
    let stream_lock = stream_exi.lock_stream();

    let app_hand = SupportedAppProtocolExi::decode_from_stream(&stream_lock)
        .expect("valid SupportedAppProtocol");

    // get app-hand protos
    let protos = app_hand.get_protocols().expect("valid document content");

    for idx in 0..protos.len() {
        let proto = &protos[idx];
        println!("  -- app-hand: proto[{}]={:?}", idx, proto);
    }
}

#[test]
fn app_hand_decode_trialog() {
    // extract from Trialog simulator handshake
    let exi_data = [
        0x01, 0xfe, 0x80, 0x01, 0x00, 0x00, 0x00, 0x44, 0x80, 0x00, 0xeb, 0xab, 0x93, 0x71, 0xd3,
        0x4b, 0x9b, 0x79, 0xd1, 0x89, 0xa9, 0x89, 0x89, 0xc1, 0xd1, 0x91, 0xd1, 0x91, 0x81, 0x89,
        0x99, 0xd2, 0x6b, 0x9b, 0x3a, 0x23, 0x2b, 0x30, 0x02, 0x00, 0x00, 0x00, 0x00, 0x01, 0xb7,
        0x57, 0x26, 0xe3, 0xa6, 0x46, 0x96, 0xe3, 0xa3, 0x73, 0x03, 0x13, 0x23, 0x13, 0xa3, 0x23,
        0x03, 0x13, 0x23, 0xa4, 0xd7, 0x36, 0x74, 0x46, 0x56, 0x60, 0x04, 0x00, 0x00, 0x08, 0x08,
        0x80,
    ];

    let stream_exi = mock_network_input(&exi_data);
    let stream_lock = stream_exi.lock_stream();

    let app_hand = SupportedAppProtocolExi::decode_from_stream(&stream_lock)
        .expect("valid SupportedAppProtocol");

    // get app-hand protos
    let protos = app_hand.get_protocols().expect("valid document content");

    for idx in 0..protos.len() {
        let proto = &protos[idx];
        println!("  -- app-hand: proto[{}]={:?}", idx, proto);
    }
}

#[test]
fn app_hand_encode_response() {
    let expected_response = [
        0x01, 0xfe, 0x80, 0x01, 0x00, 0x00, 0x00, 0x04, 0x80, 0x40, 0x00, 0x40,
    ];

    let stream_exi = ExiStream::new();
    let mut stream_lock = stream_exi.lock_stream();

    let exi_response = Some(SupportedAppProtocolRes {
        schema_id: 1,
        response_code: SupportedAppResponseCode::Success,
    });

    // encode document
    SupportedAppProtocolExi::encode_to_stream(&mut stream_lock, &exi_response)
        .expect("valid exi doc");

    // check stream attached buffer
    let length = stream_lock.get_length();
    let buffer = &stream_lock.buffer[0..length];
    println!("app-hand-protocol:{}", dump_hexa(buffer));
    assert!(buffer[0..length] == expected_response)
}
