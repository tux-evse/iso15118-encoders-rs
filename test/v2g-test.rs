use crate::mock_exi::*;
use iso15118::prelude::v2g::*;
use iso15118::prelude::*;

use std::sync::MutexGuard;

pub fn encode_to_stream<'a>(
    funcname: &str,
    stream: &'a ExiStream,
    body: V2gAppHandDoc,
) -> MutexGuard<'a, RawStream> {
    // mock network stream and encode message
    let mut stream_lock = stream.lock_stream();
    SupportedAppProtocolExi::encode_to_stream(&mut stream_lock, &body).unwrap();

    let length = stream_lock.get_cursor();
    let buffer = &stream_lock.buffer[0..length];
    println!("{}: [{}]", funcname, dump_buffer(buffer));
    stream_lock
}

pub fn decode_from_stream(stream: &MutexGuard<RawStream>) -> Result<V2gMsgBody, AfbError> {
    let stream_decode = mock_network_input(stream.get_buffer());
    let stream_lock = stream_decode.lock_stream();
    let message = SupportedAppProtocolExi::decode_from_stream(&stream_lock)?;
    Ok(message)
}

#[test]
fn supported_app_protocol_req() -> Result<(), AfbError> {
    // extract from Everest test suite
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x66, 0x80, 0x0, 0xf3, 0xab, 0x93, 0x71, 0xd3, 0x4b,
        0x9b, 0x79, 0xd1, 0x89, 0xa9, 0x89, 0x89, 0xc1, 0xd1, 0x91, 0x81, 0xd1, 0x91, 0x81, 0x91,
        0x91, 0xd2, 0x6b, 0x9b, 0x3a, 0x23, 0x2b, 0x30, 0x2, 0x0, 0x0, 0x0, 0x0, 0x1, 0xd7, 0x57,
        0x26, 0xe3, 0xa6, 0x97, 0x36, 0xf3, 0xa3, 0x13, 0x53, 0x13, 0x13, 0x83, 0xa3, 0x23, 0xa3,
        0x23, 0x3, 0x13, 0x33, 0xa4, 0xd7, 0x36, 0x74, 0x46, 0x56, 0x60, 0x4, 0x0, 0x0, 0x0, 0x0,
        0x3, 0x6e, 0xae, 0x4d, 0xc7, 0x4c, 0x8d, 0x2d, 0xc7, 0x46, 0xe6, 0x6, 0x26, 0x46, 0x27,
        0x46, 0x46, 0x6, 0x26, 0x47, 0x49, 0xae, 0x6c, 0xe8, 0x8c, 0xac, 0xc0, 0x8, 0x0, 0x0, 0x0,
        0x1,
    ];

    let proto_0 = V2G_PROTOCOLS_SUPPORTED_LIST[0];
    let proto_1 = V2G_PROTOCOLS_SUPPORTED_LIST[1];
    let proto_2: &SupportedAppProtocolConf = V2G_PROTOCOLS_SUPPORTED_LIST[2];

    // encode SupportedAppProtocolReq message
    let mut request = SupportedAppProtocolReq::new(proto_0)?;
    request.add_protocol(proto_1)?;
    request.add_protocol(proto_2)?;
    let payload = request.encode();

    // assert add protocol build correctly message
    let protocols = request.get_protocols();
    assert!(protocols[0].get_name()?.as_str() == proto_0.name);

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let v2g_body = decode_from_stream(&stream)?;
    let message = match v2g_body {
        V2gMsgBody::Request(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    let protocols = message.get_protocols();
    assert!(protocols[0].get_name()?.as_str() == proto_0.name);
    assert!(protocols[0].get_major() == proto_0.major);
    assert!(protocols[0].get_minor() == proto_0.minor);
    assert!(protocols[1].get_name()?.as_str() == proto_1.name);
    assert!(protocols[1].get_major() == proto_1.major);
    assert!(protocols[1].get_minor() == proto_1.minor);
    Ok(())
}

#[test]
fn supported_app_protocol_res() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x4, 0x80, 0x44, 0x0, 0x40,
    ];

    let schema_id = 1;
    let rcode = ResponseCode::SuccessWithMinorDeviation;

    // encode SupportedAppProtocolReq message
    let response = SupportedAppProtocolRes::new(rcode.clone(), schema_id);
    let payload = response.encode();

    assert!(response.get_schema() == schema_id);
    assert!(response.get_rcode() == rcode);

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let v2g_body = decode_from_stream(&stream)?;
    let message = match v2g_body {
        V2gMsgBody::Response(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    assert!(message.get_schema() == schema_id);
    assert!(message.get_rcode() == rcode);
    Ok(())
}
