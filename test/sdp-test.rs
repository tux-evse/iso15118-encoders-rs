use iso15118::prelude::*;
use std::net;

#[test]
fn sdp_request() -> Result<(), AfbError> {
    let payload = SdpRequest::new(SdpTransportProtocol::TCP, SdpSecurityModel::NONE).encode()?;
    let request = SdpRequest::decode(&payload)?;

    match request.check_header() {
        Ok(_) => {}
        Err(error) => panic!("{}", error),
    }

    match request.get_transport() {
        SdpTransportProtocol::TCP => {}
        _ => panic!("fail to decode transport"),
    }

    match request.get_security() {
        SdpSecurityModel::NONE => {}
        _ => panic!("fail to decode transport"),
    }

    Ok(())
}

#[test]
fn sdp_response() -> Result<(), AfbError> {
    let addr6 = net::Ipv6Addr::new(0xfe80, 1, 2, 3, 4, 5, 6, 7).octets();
    let port = 15118;

    let payload = SdpResponse::new(
        addr6,
        port,
        SdpTransportProtocol::TCP,
        SdpSecurityModel::NONE,
    )
    .encode()?;
    let response = SdpResponse::decode(&payload)?;

    match response.check_header() {
        Ok(_) => {}
        Err(error) => panic!("{}", error),
    }

    match response.get_transport() {
        SdpTransportProtocol::TCP => {}
        _ => panic!("fail to decode transport"),
    }

    match response.get_security() {
        SdpSecurityModel::NONE => {}
        _ => panic!("fail to decode transport"),
    }

    Ok(())
}

#[test]
fn sdp_decode() {
    // sdp record data bytes sample (TCP/No-TLS)
    let data_in: SdpRequestBuffer = [0x01, 0xfe, 0x90, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00];

    let request = SdpRequest::decode(&data_in).expect("valid sdp request");
    request.check_header().expect("valid header");

    println!(
        "sdp request transport:{:?} security:{:?}",
        &request.get_transport(),
        &request.get_security()
    )
}

#[test]
fn sdp_encode() {
    // sdp reponse data bytes sample for ip:[0xfe80::1:2:3:4:5:6:7] port:0xaabb
    let expected: SdpResponseBuffer = [
        0x1, 0xfe, 0x90, 0x1, 0x0, 0x0, 0x0, 0x14, 0xfe, 0x80, 0x0, 0x1, 0x0, 0x2, 0x0, 0x3, 0x0,
        0x4, 0x0, 0x5, 0x0, 0x6, 0x0, 0x7, 0xaa, 0xbb, 0x0, 0x0,
    ];

    let fake_ipv6 = net::Ipv6Addr::new(0xfe80, 1, 2, 3, 4, 5, 6, 7);
    let port = 0xaabb;

    let response = SdpResponse::new(
        fake_ipv6.octets(),
        port,
        SdpTransportProtocol::TCP,
        SdpSecurityModel::TLS,
    );
    let buffer = response.encode().expect("valid ipv6");
    println!("encoded buffer= [{}]", dump_buffer(&buffer));
    assert!(buffer == expected)
}
