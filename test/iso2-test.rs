use crate::mock_exi::*;
use iso15118::prelude::iso2::*;
use iso15118::prelude::*;
use std::sync::MutexGuard;

pub fn encode_to_stream<'a>(
    funcname: &str,
    stream: &'a ExiStream,
    body: Iso2BodyType,
) -> MutexGuard<'a, RawStream> {
    let net_session = SessionId::new(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], 8);

    // mock network stream and encode message
    let mut stream_lock = stream.lock_stream();
    Iso2MessageExi::encode_to_stream(&mut stream_lock, &body, &net_session).unwrap();

    let length = stream_lock.get_cursor();
    let buffer = &stream_lock.buffer[0..length];
    println!("{}: [{}]", funcname, dump_buffer(buffer));
    stream_lock
}

pub fn decode_from_stream(stream: &MutexGuard<RawStream>) -> Result<Iso2Payload, AfbError> {
    let stream_decode = mock_network_input(stream.get_buffer());
    let stream_lock = stream_decode.lock_stream();
    let message = Iso2Payload::decode_from_stream(&stream_lock)?;
    Ok(message)
}

#[test]
// cargo test --package iso15118 --test test-v2g --  test_iso2::enum_rename --exact --nocapture
fn enum_rename() {
    println!(
        "MessageTagId:SessionSetupReq= {}",
        MessageTagId::SessionSetupReq.to_json().unwrap()
    );
    println!(
        "MessageTagId:SessionSetupReq= {:?}",
        MessageTagId::from_json("\"session_setup_req\"").unwrap()
    );
}
#[test]
fn session_setup_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x15, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xd0, 0x18, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x0,
    ];

    // Encoding API
    let setup_tst = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    let payload = SessionSetupRequest::new(&setup_tst)?.encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::SessionSetupReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let setup_rec = payload.get_id();
    assert!(setup_tst == setup_rec);

    Ok(())
}

#[test]
fn session_setup_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1c, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xe0, 0x0, 0x39, 0xd1, 0xd5, 0xe0, 0xb5, 0x95, 0xd9, 0xcd, 0x94,
        0xb4, 0xc0, 0xc0, 0xc4, 0x80,
    ];

    // Encoding API
    let evse_tst = "tux-evse-001";
    let code_tst = ResponseCode::Ok;
    let payload = iso2::SessionSetupResponse::new(evse_tst, code_tst.clone())?.encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::SessionSetupRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let evse_rec = payload.get_id()?;
    let code_rec = payload.get_rcode();
    let _time_stamp = payload.get_time_stamp();

    assert!(evse_tst == evse_rec);
    assert!(code_tst == code_rec);

    Ok(())
}

#[test]
fn service_discovery_request() -> Result<(), AfbError> {
    // empty scope
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1c, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xb0, 0x1c, 0xe6, 0xc2, 0xda, 0xe0, 0xd8, 0xca, 0x5a, 0xe6, 0xc6,
        0xde, 0xe0, 0xca, 0x0, 0x0,
    ];

    // Encoding API
    let scope_tst = "sample-scope";
    let category_tst = ServiceCategory::EvCharger;
    let payload = ServiceDiscoveryRequest::new()
        .set_scope(scope_tst)?
        .set_category(category_tst.clone())
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ServiceDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let scope_rec = payload.get_scope().unwrap();
    let category_rec = payload.get_category().unwrap();

    // assert input==output
    assert!(scope_tst == scope_rec);
    assert!(category_tst == category_rec);

    Ok(())
}

#[test]
fn service_discovery_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x46, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xc0, 0x0, 0x8, 0x0, 0x0, 0x29, 0x51, 0xd5, 0xe0, 0xb5, 0x15, 0xd9,
        0xcd, 0x94, 0x0, 0x12, 0x92, 0xde, 0xa8, 0x5c, 0xc4, 0xf4, 0xd0, 0x0, 0x1, 0x10, 0xe, 0x0,
        0x15, 0x31, 0x51, 0x14, 0x20, 0x12, 0x9c, 0xca, 0xe8, 0xee, 0xde, 0xe4, 0xd6, 0x20, 0xe,
        0x80, 0x2a, 0x7a, 0xa2, 0x8, 0xc0, 0x21, 0x55, 0xc1, 0x91, 0x85, 0xd1, 0x94, 0x44, 0x0,
    ];

    let rcode_tst = ResponseCode::Ok;
    let charging_tst = ServiceCharging::new("Tux-Evse", "IoT.bzh", false);
    let payment_tst0 = PaymentOption::Contract;
    let payment_tst1 = PaymentOption::External;
    let service_tst0 = ServiceOther::new(56, "LTE", "Network", ServiceCategory::Internet, true);
    let service_tst1 = ServiceOther::new(29, "OTA", "Update", ServiceCategory::Other, true);
    let transfer_tst0 = EngyTransfertMode::AcSinglePhase;
    let transfer_tst1 = EngyTransfertMode::DcBasic;

    let payload = ServiceDiscoveryResponse::new(rcode_tst.clone())
        .set_charging(&charging_tst)?
        .add_transfer(transfer_tst0.clone())?
        .add_transfer(transfer_tst1.clone())?
        .add_payment(payment_tst0.clone())?
        .add_payment(payment_tst1.clone())?
        .add_service(&service_tst0)?
        .add_service(&service_tst1)?
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ServiceDiscoveryRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_rec = payload.get_rcode();
    let charging_rec = payload.get_charging()?;
    let transfers_rec = payload.get_transfers()?;
    let payments_rec = payload.get_payments();
    let services_rec = payload.get_services()?;

    // assert input==output
    assert!(rcode_rec == rcode_tst);
    assert!(charging_rec.get_name() == charging_tst.get_name());
    assert!(charging_rec.get_scope() == charging_tst.get_scope());
    assert!(charging_rec.get_isfree() == charging_tst.get_isfree());
    assert!(transfers_rec[0] == transfer_tst0);
    assert!(transfers_rec[1] == transfer_tst1);
    assert!(payments_rec[0] == payment_tst0);
    assert!(payments_rec[1] == payment_tst1);
    assert!(services_rec[0].get_id() == service_tst0.get_id());
    assert!(services_rec[0].get_name() == service_tst0.get_name());
    assert!(services_rec[0].get_scope() == service_tst0.get_scope());
    assert!(services_rec[0].get_isfree() == service_tst0.get_isfree());
    assert!(services_rec[0].get_category() == service_tst0.get_category());
    assert!(services_rec[1].get_id() == service_tst1.get_id());
    assert!(services_rec[1].get_name() == service_tst1.get_name());
    assert!(services_rec[1].get_scope() == service_tst1.get_scope());
    assert!(services_rec[1].get_isfree() == service_tst1.get_isfree());
    assert!(services_rec[1].get_category() == service_tst1.get_category());

    Ok(())
}

#[test]
fn service_detail_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x10, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x93, 0x48, 0x24, 0x0,
    ];

    let id_tst = 1234;

    // Encoding api
    let payload = ServiceDetailRequest::new(id_tst).encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ServiceDetailReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_rec = payload.get_id();

    // assert input == output
    assert!(id_tst == id_rec);

    Ok(())
}

#[test]
fn service_detail_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x5d, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xa0, 0x0, 0xe0, 0x0, 0x8, 0x7, 0x70, 0x72, 0x6d, 0x5f, 0x31, 0x43,
        0xd8, 0x1, 0xdc, 0x1c, 0x9b, 0x57, 0xcc, 0xa8, 0x21, 0xcd, 0xb9, 0xbd, 0xbd, 0xc1, 0xe4,
        0x0, 0xee, 0xe, 0x4d, 0xab, 0xe6, 0x70, 0x82, 0x7, 0x80, 0x8, 0x40, 0x4, 0x1, 0xdc, 0x1c,
        0x9b, 0x57, 0xcc, 0x51, 0xa4, 0x12, 0x0, 0x77, 0x7, 0x26, 0xd5, 0xf3, 0x2a, 0xe, 0x4d,
        0x6d, 0x65, 0x20, 0x4b, 0x65, 0x72, 0x6d, 0x69, 0x63, 0x68, 0x75, 0x0, 0x3b, 0x83, 0x93,
        0x6a, 0xf9, 0x9c, 0x20, 0x60, 0x14, 0x14, 0x0,
    ];

    let mut param_tst0 = ParamSet::new(1);
    param_tst0.add_param("prm_1", Iso2ParamValue::Int16(123))?;
    param_tst0.add_param("prm_2", Iso2ParamValue::Text("snoopy".to_string()))?;
    param_tst0.add_param(
        "prm_3",
        Iso2ParamValue::PhyValue(PhysicalValue::new(240, 1, Isp2PhysicalUnit::Volt)),
    )?;

    let mut param_tst1 = ParamSet::new(2);
    param_tst1.add_param("prm_1", Iso2ParamValue::Int16(1234))?;
    param_tst1.add_param("prm_2", Iso2ParamValue::Text("Mme Kermichu".to_string()))?;
    param_tst1.add_param(
        "prm_3",
        Iso2ParamValue::PhyValue(PhysicalValue::new(10, 1, Isp2PhysicalUnit::Ampere)),
    )?;

    let id_tst = 56;
    let rcode_tst = ResponseCode::Ok;

    // Encoding api
    let mut payload = ServiceDetailResponse::new(id_tst, rcode_tst.clone());
    payload.add_pset(&param_tst0)?;
    payload.add_pset(&param_tst1)?;

    // keep track of input psets for assert check
    let psets_tst = payload.get_psets();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload.encode());
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ServiceDetailRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_rec = payload.get_id();
    let rcode_rec = payload.get_rcode();
    let psets_rec = payload.get_psets();

    // assert input == output
    assert!(id_tst == id_rec);
    assert!(rcode_tst == rcode_rec);
    assert!(psets_rec.len() == psets_tst.len());
    for idx in 0..psets_rec.len() {
        let set_rec = psets_rec[idx].clone();
        let set_tst = psets_tst[idx].clone();
        assert!(set_rec.get_id() == set_tst.get_id());

        let prms_rec = set_rec.get_params()?;
        let prms_tst = set_tst.get_params()?;
        assert!(prms_rec.len() == prms_tst.len());

        for jdx in 0..prms_rec.len() {
            assert!(prms_rec[jdx].get_name() == prms_tst[jdx].get_name());
            let value_rec = prms_rec[jdx].get_value();
            let value_tst = prms_tst[jdx].get_value();

            match value_rec {
                Iso2ParamValue::Int16(rec) => match value_tst {
                    Iso2ParamValue::Int16(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_tst:{:?} != value_rec:{}", value_tst, rec),
                },
                Iso2ParamValue::Text(rec) => match value_tst {
                    Iso2ParamValue::Text(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_tst:{:?} != value_rec:{}", value_tst, rec),
                },
                Iso2ParamValue::PhyValue(rec) => match value_tst {
                    Iso2ParamValue::PhyValue(tst) => {
                        assert!(rec.get_unit() == tst.get_unit());
                        assert!(rec.get_multiplier() == tst.get_multiplier());
                        assert!(rec.get_value() == tst.get_value());
                    }
                    _ => panic!(
                        "unexpected value_tst:{:?} != value_rec:{:?}",
                        value_tst, rec
                    ),
                },
                _ => panic!("unexpected decoded param value:{:?} type", value_rec),
            }
        }
    }
    Ok(())
}

#[test]
fn authorization_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1c, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x0, 0x29, 0xd1, 0xd5, 0xe0, 0xb5, 0x95, 0xd9, 0xcd, 0x94, 0x2,
        0x0, 0x81, 0x1, 0x82, 0x0,
    ];

    let id_tst = "tux-evse";
    let challenge_tst = [0x1, 0x2, 0x3, 0x4];

    // Encoding api
    let payload = AuthorizationRequest::new()
        .set_id(id_tst)?
        .set_challenge(&challenge_tst)?
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::AuthorizationReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_rec = payload.get_id().unwrap();
    let challenge_rec = payload.get_challenge().unwrap();

    // assert input == output
    assert!(id_tst == id_rec);
    assert!(challenge_tst == challenge_rec);

    Ok(())
}

#[test]
fn authorization_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xf, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x10, 0x20, 0x0,
    ];

    let rcode_tst = ResponseCode::NewSession;
    let processing_tst = EvseProcessing::Finished;

    // Encoding api
    let payload = AuthorizationResponse::new(rcode_tst.clone(), processing_tst.clone()).encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::AuthorizationRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_rec = payload.get_rcode();
    let processing_rec = payload.get_processing();

    // assert input == output
    assert!(rcode_rec == rcode_tst);
    assert!(processing_rec == processing_tst);

    Ok(())
}

#[test]
fn cable_check_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x10, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x31, 0x0, 0x8, 0x0,
    ];

    // Encoding api
    let ready_tst = true;
    let error_tst = DcEvErrorCode::NoError;
    let evresssoc_tst: i8 = 16;
    let status_tst = DcEvStatusType::new(ready_tst, error_tst.clone(), evresssoc_tst);
    let payload = CableCheckRequest::new(&status_tst).encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::CableCheckReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let status_rec = payload.get_status();

    // assert input == output
    assert!(status_rec.get_ready() == ready_tst);
    assert!(status_rec.get_error() == error_tst);
    assert!(status_rec.get_evresssoc() == evresssoc_tst);

    Ok(())
}

#[test]
fn cable_check_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x13, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x40, 0x21, 0x40, 0x2, 0x22, 0x10, 0x40,
    ];

    // Encoding api
    let code_tst = ResponseCode::NewSession;
    let processing_tst = EvseProcessing::Ongoing;

    let error_tst = DcEvseErrorCode::Ready;
    let notification_tst = EvseNotification::ReNegociation;
    let delay_tst = 160;
    let status_tst = DcEvseStatusType::new(error_tst.clone(), notification_tst.clone(), delay_tst);

    let payload =
        CableCheckResponse::new(code_tst.clone(), &status_tst, processing_tst.clone()).encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::CableCheckRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let code_rec = payload.get_code();
    let status_rec = payload.get_status();
    let processing_rec = payload.get_processing();

    // assert input == output
    assert!(code_rec == code_tst);
    assert!(processing_rec == processing_tst);
    assert!(status_rec.get_notification() == notification_tst);
    assert!(status_rec.get_delay() == delay_tst);
    assert!(status_rec.get_error() == error_tst);

    Ok(())
}

#[test]
fn certificate_install_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x21, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x50, 0x53, 0xa3, 0xab, 0xc1, 0x6b, 0x2b, 0xb3, 0x9b, 0x28, 0xc,
        0x2, 0x4, 0x6, 0x8, 0xa, 0xc, 0x0, 0x20, 0x0, 0x10,
    ];

    // Encoding api
    let issuer_tst0 = "IoT.bzh";
    let serial_tst0 = 1234;
    let issuer_tst1 = "Redpesk.bzh";
    let serial_tst1 = 5678;

    let mut list_tst = CertificateRootList::new(issuer_tst0, serial_tst0)?;
    list_tst.add_cert(issuer_tst1, serial_tst1)?;

    let id_tst = "tux-evse";
    let provisioning_tst = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];

    let payload = CertificateInstallRequest::new(id_tst, &provisioning_tst, &list_tst)?.encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::CertificateInstallReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_rec = payload.get_id()?;
    let provisioning_rec = payload.get_provisioning();
    let certs_list_rec = payload.get_certs_list().get_certs()?;
    let certs_list_tst = list_tst.get_certs()?;

    // assert input == output
    assert!(id_rec == id_tst);
    assert!(provisioning_rec == provisioning_tst);
    assert!(certs_list_rec.len() == certs_list_tst.len());

    for idx in 0..certs_list_rec.len() {
        let cert_rec = certs_list_rec[idx].clone();
        let cert_tst = certs_list_tst[idx].clone();

        assert!(cert_rec.get_issuer() == cert_tst.get_issuer());
        assert!(cert_rec.get_serial() == cert_tst.get_serial());
    }

    Ok(())
}

#[test]
fn certificate_install_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xaf, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x60, 0x20, 0x1c, 0x86, 0xca, 0xe4, 0xe8, 0x5a, 0xa8, 0xea, 0xf0,
        0x8a, 0xec, 0xa6, 0x8a, 0x3, 0x0, 0x81, 0x1, 0x82, 0x2, 0x83, 0x0, 0x18, 0x44, 0x48, 0x4c,
        0x50, 0x54, 0x58, 0x1, 0x88, 0x48, 0x88, 0xc9, 0x9, 0x49, 0x88, 0x9, 0x21, 0xb7, 0xb7,
        0x3a, 0x39, 0x30, 0xb1, 0xba, 0x16, 0xaa, 0x3a, 0xbc, 0x22, 0xbb, 0x29, 0xa2, 0x80, 0xd4,
        0x34, 0x54, 0x74, 0x94, 0xb4, 0xc0, 0x6, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0x0, 0x6c,
        0x1c, 0x2c, 0x3c, 0x4c, 0x5c, 0x62, 0x4, 0x54, 0x1c, 0x9a, 0x5d, 0x98, 0x5d, 0x19, 0x57,
        0xd5, 0x1d, 0x5e, 0x11, 0x5d, 0x94, 0xd9, 0x40, 0xda, 0x3a, 0x5a, 0x7a, 0x9a, 0xba, 0xc0,
        0x41, 0xc1, 0xd5, 0x89, 0xb1, 0xa5, 0x8d, 0x7d, 0x51, 0xd5, 0xe1, 0x15, 0xd9, 0x4d, 0x94,
        0xd, 0xc3, 0xc5, 0xc7, 0xc9, 0xcb, 0xcc, 0x3, 0xd9, 0x5b, 0x58, 0x5a, 0x59, 0x17, 0xd5,
        0x1d, 0x5e, 0x11, 0x5d, 0x94, 0xd1, 0x43, 0x2d, 0xaf, 0x24, 0xc, 0xad, 0xac, 0x2d, 0x2c,
        0x84, 0xe, 0x8c, 0xae, 0x6e, 0x8d, 0x2d, 0xcc, 0xe4, 0xe, 0x6e, 0x8e, 0x4d, 0x2d, 0xcc,
        0xe0,
    ];

    // Encoding api
    let cert_id_tst = "Cert-TuxEvSE";
    let cert_main_tst = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let cert_sub_tst0 = [0x11, 0x12, 0x13, 0x14, 0x15, 0x16];
    let cert_sub_tst1 = [0x21, 0x22, 0x23, 0x24, 0x25, 0x26];
    let mut cert_chain_tst = CertificateChainType::new(cert_id_tst, &cert_main_tst)?;
    cert_chain_tst.add_subcert(&cert_sub_tst0)?;
    cert_chain_tst.add_subcert(&cert_sub_tst1)?;

    let contract_id_tst = "Contract-TuxEvSE";
    let contract_main_tst = [0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6];
    let contract_sub_tst0 = [0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6];
    let contract_sub_tst1 = [0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6];
    let mut contract_chain_tst = CertificateChainType::new(contract_id_tst, &contract_main_tst)?;
    contract_chain_tst.add_subcert(&contract_sub_tst0)?;
    contract_chain_tst.add_subcert(&contract_sub_tst1)?;

    let private_id_tst = "Private_TuxEvSe";
    let private_data_tst = [0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6];
    let private_key_tst = PrivateKeyType::new(private_id_tst, &private_data_tst)?;

    let public_id_tst = "public_TuxEvSe";
    let public_data_tst = [0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6];
    let public_key_tst = DhPublicKeyType::new(public_id_tst, &public_data_tst)?;

    let emaid_id_tst = "emaid_TuxEvSE";
    let emaid_str_tst = "my emaid testing string";
    let emaid_tst = EmaidType::new(emaid_id_tst, emaid_str_tst)?;

    let rcode_tst = ResponseCode::NewSession;

    let payload = CertificateInstallResponse::new(
        rcode_tst.clone(),
        &cert_chain_tst,
        &contract_chain_tst,
        &private_key_tst,
        &public_key_tst,
        &emaid_tst,
    )
    .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::CertificateInstallRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_rec = payload.get_rcode();
    let cert_chain_rec = payload.get_provisioning_chain();
    let contract_chain_rec = payload.get_contract_chain();
    let private_key_rec = payload.get_private_key();
    let public_key_rec = payload.get_public_key();
    let emaid_rec = payload.get_emaid();

    // assert input == output
    assert!(rcode_rec == rcode_tst);
    assert!(cert_chain_rec.get_id() == cert_chain_tst.get_id());
    assert!(cert_chain_rec.get_cert() == cert_chain_tst.get_cert());
    assert!(contract_chain_rec.get_id() == contract_chain_tst.get_id());
    assert!(contract_chain_rec.get_cert() == contract_chain_tst.get_cert());
    assert!(private_key_rec.get_id()? == private_key_tst.get_id()?);
    assert!(private_key_rec.get_data() == private_key_tst.get_data());
    assert!(public_key_rec.get_id()? == public_key_tst.get_id()?);
    assert!(private_key_rec.get_data() == private_key_tst.get_data());
    assert!(emaid_rec.get_id()? == emaid_tst.get_id()?);
    assert!(emaid_rec.get_data()? == emaid_tst.get_data()?);
    let certs_sub_rec = cert_chain_rec.get_subcerts();
    assert!(certs_sub_rec[0] == cert_sub_tst0);
    assert!(certs_sub_rec[1] == cert_sub_tst1);

    Ok(())
}

#[test]
fn current_demand_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1d, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xd1, 0x0, 0x0, 0x82, 0x6, 0xa, 0x0, 0x20, 0x81, 0x40, 0xc, 0x35,
        0x10, 0x40, 0x90, 0x3, 0x0,
    ];

    // Encoding API
    let dc_ready = true;
    let dc_error = DcEvErrorCode::NoError;
    let dc_evresssoc = 1;

    let dc_status = DcEvStatusType::new(dc_ready, dc_error, dc_evresssoc);
    let dc_current = PhysicalValue::new(80, 1, Isp2PhysicalUnit::Ampere);
    let dc_tension = PhysicalValue::new(400, 1, Isp2PhysicalUnit::Volt);
    let dc_limit = PhysicalValue::new(800, 1, Isp2PhysicalUnit::Volt);
    let dc_complete = true;

    let payload = CurrentDemandRequest::new(dc_status, &dc_current, &dc_tension, dc_complete)
        .set_voltage_limit(&dc_limit)
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::CurrentDemandReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(dc_complete == payload.get_charging_complete());
    let current = payload.get_current_target();
    assert!(current.get_unit() == Isp2PhysicalUnit::Ampere);
    assert!(current.get_value() == 80);
    assert!(current.get_multiplier() == 1);
    let tension = payload.get_voltage_target();
    assert!(tension.get_value() == 400);
    assert!(payload.get_voltage_limit().unwrap().get_value() == 800);

    Ok(())
}

#[test]
fn charging_status_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xd, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xb0,
    ];

    // Encoding API
    let payload = ChargingStatusRequest::new().encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let _payload = match message.get_payload() {
        Iso2MessageBody::ChargingStatusReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API (nothing to do)

    Ok(())
}

#[test]
fn metering_receipt_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x3e, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xf0, 0x3d, 0x99, 0xd5, 0xb1, 0xd5, 0xc0, 0xb5, 0xa5, 0xbd, 0xd0,
        0xb5, 0x89, 0xe9, 0xa0, 0x6, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x3, 0xf0, 0xa, 0x74, 0x75,
        0x78, 0x2d, 0x65, 0x76, 0x73, 0x65, 0x2, 0x0, 0x1, 0x42, 0x82, 0xc3, 0x3, 0x43, 0x81, 0xfe,
        0x2, 0xa, 0x5d, 0x9f, 0x43, 0xa0, 0x0,
    ];

    // Encoding API
    let session_id = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    let meeter_id = "tux-evse";
    let payload_id = "fulup-iot-bzh";
    let signature = [0xa, 0xb, 0xc, 0xd, 0xe];

    let mut meter_info = MeterInfoType::new(meeter_id)?;
    meter_info
        .set_reading(64)
        .set_status(255)
        .set_tmeter(123546789)
        .set_sig(&signature)?;

    let payload = MeteringReceiptRequest::new(&session_id, &meter_info)?
        .set_id(payload_id)?
        .set_tupple_id(64)
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::MeteringReceiptReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_id().unwrap() == payload_id);
    assert!(payload.get_session_id() == session_id);
    assert!(payload.get_tuple_id().unwrap() == 64);
    let info = payload.get_info();
    assert!(info.get_reading().unwrap() == 64);
    assert!(info.get_id()? == meeter_id);
    assert!(info.get_status().unwrap() == 255);
    assert!(info.get_tmeter().unwrap() == 123546789);
    assert!(info.get_sig().unwrap() == signature);

    Ok(())
}

#[test]
fn ac_param_discovery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x20, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x90, 0x20, 0x0, 0x1a, 0x41, 0x21, 0x46, 0x1, 0x40, 0x41, 0x2,
        0x40, 0xc, 0x10, 0x30, 0x40, 0x4, 0xc, 0x2, 0x80,
    ];
    let ea_mount = PhysicalValue::new(20, 10, Isp2PhysicalUnit::Wh);
    let ac_max_voltage = PhysicalValue::new(400, 1, Isp2PhysicalUnit::Volt);
    let ac_max_current = PhysicalValue::new(64, 1, Isp2PhysicalUnit::Ampere);
    let ac_min_current = PhysicalValue::new(10, 1, Isp2PhysicalUnit::Ampere);
    let mut ac_params =
        AcEvChargeParam::new(&ea_mount, &ac_max_voltage, &ac_max_current, &ac_min_current)?;
    ac_params.set_departure_time(1234);

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::AcSinglePhase)
        .set_max_schedule_tuple(16)
        .set_ac_charge_param(&ac_params)?
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ParamDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_max_schedule_tuple().unwrap() == 16);
    let ac_value = payload.get_ac_charge_param().unwrap();
    assert!(ac_value.get_departure_time().unwrap() == 1234);
    assert!(ac_value.get_max_current().get_value() == 64);
    assert!(ac_value.get_min_current().get_value() == 10);
    assert!(ac_value.get_max_voltage().get_value() == 400);

    Ok(())
}

#[test]
fn dc_param_discovery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1b, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x90, 0x20, 0x11, 0x48, 0x0, 0x4, 0x10, 0x30, 0x64, 0x12, 0x8,
        0x14, 0x0, 0xc4, 0x0,
    ];

    let dc_max_voltage = PhysicalValue::new(800, 1, Isp2PhysicalUnit::Volt);
    let dc_max_current = PhysicalValue::new(100, 1, Isp2PhysicalUnit::Ampere);
    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::NoError, 1);
    let dc_params = DcEvChargeParam::new(dc_status, dc_max_current, dc_max_voltage)?;

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::DcBasic)
        .set_max_schedule_tuple(16)
        .set_dc_charge_param(&dc_params)?
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ParamDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_max_schedule_tuple().unwrap() == 16);
    let dc_value = payload.get_dc_charge_param().unwrap();
    assert!(dc_value.get_max_current().get_value() == 100);
    assert!(dc_value.get_max_voltage().get_value() == 800);
    assert!(dc_value.get_status().get_error() == DcEvErrorCode::NoError);
    assert!(dc_value.get_status().get_ready() == true);
    Ok(())
}

#[test]
fn ev_param_discovery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x2e, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x90, 0x20, 0x22, 0x1a, 0x41, 0x22, 0x40, 0x0, 0x20, 0x81, 0x83,
        0x20, 0x90, 0x40, 0xa0, 0x6, 0x20, 0x69, 0x4, 0x85, 0x18, 0x5, 0x1, 0x4, 0x9, 0x0, 0x30,
        0x40, 0xc1, 0x0, 0x10, 0x30, 0xa, 0x0,
    ];

    let ea_mount = PhysicalValue::new(20, 10, Isp2PhysicalUnit::Wh);
    let ac_max_voltage = PhysicalValue::new(400, 1, Isp2PhysicalUnit::Volt);
    let ac_max_current = PhysicalValue::new(64, 1, Isp2PhysicalUnit::Ampere);
    let ac_min_current = PhysicalValue::new(10, 1, Isp2PhysicalUnit::Ampere);
    let mut ac_params =
        AcEvChargeParam::new(&ea_mount, &ac_max_voltage, &ac_max_current, &ac_min_current)?;
    ac_params.set_departure_time(1234);

    let dc_max_voltage = PhysicalValue::new(800, 1, Isp2PhysicalUnit::Volt);
    let dc_max_current = PhysicalValue::new(100, 1, Isp2PhysicalUnit::Ampere);
    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::NoError, 1);
    let dc_params = DcEvChargeParam::new(dc_status, dc_max_current, dc_max_voltage)?;

    let mut charge_param = EvChargeParam::new(&ac_params, &dc_params);
    charge_param.set_departure_time(1234);

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::DcCombo)
        .set_max_schedule_tuple(16)
        .set_charge_param(&charge_param)?
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let exi = ExiStream::new();
    let stream = encode_to_stream(func_name!(), &exi, payload);
    assert!(expected_response == stream.get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(&stream)?;
    let payload = match message.get_payload() {
        Iso2MessageBody::ParamDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_max_schedule_tuple().unwrap() == 16);
    // check DC params
    let params = payload.get_charge_param().unwrap();
    let dc_value = params.get_dc_param();
    assert!(dc_value.get_max_current().get_value() == 100);
    assert!(dc_value.get_max_voltage().get_value() == 800);
    assert!(dc_value.get_status().get_error() == DcEvErrorCode::NoError);
    assert!(dc_value.get_status().get_ready() == true);
    // check ac params
    let ac_value = params.get_ac_param();
    assert!(ac_value.get_departure_time().unwrap() == 1234);
    assert!(ac_value.get_max_current().get_value() == 64);
    assert!(ac_value.get_min_current().get_value() == 10);
    assert!(ac_value.get_max_voltage().get_value() == 400);
    Ok(())
}
