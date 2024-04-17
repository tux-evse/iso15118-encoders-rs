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

// CertificateSupportAppProtocol
// "\x98\xfa\x9b\x84\xb1\x3f\x00\x01\x87\x05\xbb\xc8\x86\xdd\x60\x0b" \
// "\xd0\x12\x03\xe1\x06\x40\xfe\x80\x00\x00\x00\x00\x00\x00\x02\x01" \
// "\x87\xff\xfe\x05\xbb\xc8\xfe\x80\x00\x00\x00\x00\x00\x00\x29\xd0" \
// "\x26\xf7\xe7\x8a\xa2\x32\xd2\x34\xfa\x6d\x01\xb9\xdc\x0e\x10\xca" \
// "\x3a\xde\x80\x18\x08\xb4\xf2\x50\x00\x00\x01\x01\x08\x0a\x00\x00" \
// "\x32\x03\x3b\x34\xdf\x0a\x9a\xb8\x05\xfa\x87\x36\x4d\x72\xd0\x43" \
// "\x38\xfb\xab\x21\x8a\xd8\x08\xda\xe7\xc4\xb6\x8c\x71\x81\x2a\xa9" \
// "\x67\xa5\xef\x46\xdc\xae\x52\x99\x41\xa0\xf2\xcc\xd5\xb7\xe2\x15" \
// "\x44\x8a\x5b\xe5\x17\x37\x65\xa6\xa6\x9d\x27\xa7\x5a\x20\x79\x30" \
// "\x82\x9a\x45\xd3\xa8\xaa\xe7\xe3\x17\xf5\x81\xe9\x37\x69\x61\x3c" \
// "\x8e\xe2\xf2\xcf\x5a\x65\x3a\xff\x42\xb6\x6e\xeb\x24\x13\x59\x6f" \
// "\x8d\x9e\x33\x41\x55\x93\x6b\x0b\x68\x9f\xf4\x9c\x43\x2d\x18\x7c" \
// "\xdf\xe7\x26\x5b\x72\x9f\xd1\x46\xa9\x22\xe6\x6c\x62\x97\x22\x58" \
// "\xc2\x66\x39\x88\xe7\x7e\x10\xe7\x32\xb1\xad\x7a\x8d\xe1\xdf\x65" \
// "\xee\x2e\x65\xce\x74\x7e\x9f\xae\x9c\xda\x29\xac\xb3\xdd\x22\x53" \
// "\xfd\x10\xe4\x3f\x50\x42\x81\x7d\xb2\x86\x4f\x4a\x98\x48\x62\x9a" \
// "\xae\xcb\x87\xbd\x23\xf7\xf8\xb8\x58\x1c\xf9\x1f\xd6\xd1\x9d\xaf" \
// "\xb7\x69\x7b\x7e\xc4\xb5\x30\x86\xa8\xa6\xad\xb9\x17\x5b\x42\x39" \
// "\xed\x00\x36\xf7\xa8\x37\xca\x04\xc4\xe4\xdc\xe0\xe2\x6e\xb3\x1b" \
// "\x6c\xcc\x7b\x34\xa3\x69\x45\x4b\x33\xbb\x8b\xf4\x4e\x18\x1d\xa7" \
// "\x1c\xc0\xd9\x74\x25\xfe\xd3\xd8\xc0\x71\x89\x42\xd6\x56\xc3\x15" \
// "\xc8\x44\xb8\xe9\x5b\xc8\xe0\xec\x0a\x20\xa4\xbb\xad\xe5\x80\x27" \
// "\x0a\x27\x60\x32\xa0\x1a\xcd\x76\x60\xce\x40\xfa\x12\x0d\x03\x91" \
// "\xcc\x35\x65\xeb\x8d\x3a\xb0\xfa\xa7\x86\x45\x6a\x16\x55\x61\xb9" \
// "\x7c\x6c\xe7\x41\x34\xb1\x03\x32\xd1\x28\x0d\x5b\xff\x89\x89\x02" \
// "\xf4\xee\xfa\x26\x42\xf1\xca\x97\x3c\xa1\x7f\x50\x55\xea\xa6\x0a" \
// "\x48\x06\xe3\x95\x05\x82\xf8\x68\x3b\x8f\x24\xee\x8a\xcc\xbc\x80" \
// "\x8e\x62\x24\xe8\x7f\x64\xf8\x15\xec\x29\x24\xf9\x21\x36\x2e\x8e" \
// "\x5e\x53\xf3\x7c\xb6\xc0\xe8\xf4\x07\xbd\xc3\x93\x3b\x1f\x24\x10" \
// "\xd2\x44\x69\xa9\x19\xe6\x62\x7c\x7a\xb7\x02\x38\x52\x83\x30\xa6" \
// "\x3c\xe5\x1e\x37\x88\x40\x3e\xe8\x89\xa6\x88\xd4\xd1\xdd\x3c\xea" \
// "\x71\x21\x03\xfd\xf6\xb2\x99\x10\xf6\x7a\x26\xb2\x85\x65\x78\x48" \
// "\x4b\x3b\x81\x5b\xbb\xf7\xb6\x6b\x07\x2e\x17\x7c\xfe\x79\x16\xd5" \
// "\xd6\x0c\xeb\x3b\x12\x63\x84\x72\x2f\x25\x49\x69\x34\x0a\xdf\x9d" \
// "\x1e\x35\xd2\x3d\x83\x10\x1b\x34\x82\xda\xbd\xd9\x95\xab\x31\x7f" \
// "\xe4\xca\xa6\xd2\xb8\x94\xe1\xe7\xfe\x44\x8b\xf6\xb5\x56\x86\x5d" \
// "\xca\xcd\xac\xba\x53\xe2\xf4\x41\xcb\x98\x96\x6d\x49\x04\xa2\x64" \
// "\xd5\x6b\x16\xd7\x16\x2d\x03\x22\xb0\x52\x2f\x39\xe4\x98\x92\x4c" \
// "\xe2\xc0\xf0\x91\xa5\x43\xa6\xd5\x53\x5e\xe0\x65\x83\x35\x6d\x2f" \
// "\x74\x20\x52\x86\xe0\x1d\x1c\x27\x26\x73\xd7\xc3\xfc\x53\x5f\x84" \
// "\x82\x40\x9e\x3f\x7f\x50\x5d\x7a\x0c\xc0\xcb\x6a\x88\x7d\x91\xfe" \
// "\x57\x80\x93\xc5\xe8\x91\xac\x0a\x39\x6b\x89\x76\x95\x8b\x34\x85" \
// "\x12\x11\x52\x5f\x80\xa7\xdb\x82\xf5\x31\x5f\x84\xe3\xa6\x47\xbb" \
// "\x28\x6a\x93\xa2\xa8\x64\x34\x3c\x68\xdc\x59\x5a\x95\x67\x82\x06" \
// "\x83\x6a\x17\x03\x03\x00\xa4\xf2\x36\x6e\x04\x3f\x19\xe4\x4a\x88" \
// "\x67\xa5\x25\x3d\x9f\x54\x2a\x76\x29\xfa\x25\xcb\xc7\x44\x43\x03" \
// "\xa6\xbb\x15\xd6\xcc\x76\x1c\x4e\x69\x52\x8c\x78\xe2\xb5\xfa\xa4" \
// "\xa6\x7e\x2c\x09\xd0\xed\x93\xe1\x1d\x87\x45\x18\xc4\x47\x1f\x5f" \
// "\xd3\x13\x6d\x11\x01\x2f\x8b\x00\x2b\x78\x3a\x38\x97\x3c\x88\x1d" \
// "\x2a\xc2\x98\xb8\x9e\x73\x9b\xa8\xd3\x76\xd3\xe7\x4d\xe6\xd3\xf8" \
// "\xca\xc0\xc9\xd9\x02\x5f\xdb\x85\x31\xf6\xd4\x37\x05\x6a\xb6\x47" \
// "\x7d\x67\x5e\xb4\xa7\xc5\x6b\x45\x89\x0d\xea\xd1\xe5\xdc\x77\xd9" \
// "\xe5\x7e\x0e\x36\x63\x0e\x48\xaa\x75\xd0\x06\x55\x59\x02\xc3\x56" \
// "\xda\x0c\x17\x82\xe0\xff\xa5\xd8\x2f\x10\xc9\xb3\x7f\x58\x11\xee" \
// "\x80\xf0\xc5\x09\x4f\xba\x6f\x2e\xd7\x3e\xe0\x17\x03\x03\x00\x45" \
// "\x54\x4f\xa9\x64\xd6\xbc\xe8\x0a\x9c\xdd\xa2\x86\x9b\x28\x4a\xa2" \
// "\xc1\xce\x4d\x7c\xe8\xa2\xa4\xb2\xd5\x8c\xbe\x9b\xbb\x4d\xe4\xde" \
// "\xe3\x6a\xf3\x0a\xcf\xbc\x32\x85\xab\x52\x6a\xec\x40\xf3\x1a\x75" \
// "\x61\x0f\x1e\x8b\xf4\xd7\xdc\xe6\x12\x1f\xde\x40\x47\x56\x1c\x54" \
// "\x56\xbf\x63\xa0\xa0\x17\x03\x03\x00\x5d\xd7\x86\xcf\x9b\x26\x66" \
// "\xbf\x43\x54\x73\x23\x4d\x84\xc6\x89\x69\x1d\x76\x4d\x84\x43\xb8" \
// "\x88\xd2\x76\x59\xc8\x67\x2d\x18\x70\x72\x77\x1a\xc3\xca\x8c\x1f" \
// "\xce\x32\xe0\xbe\x19\x26\xbc\x66\xd0\xbf\x2e\x6c\x74\xc5\x14\xc3" \
// "\x46\x3c\x6d\x81\x46\x77\xe8\xad\x63\xfe\xa4\x91\xdc\x0a\x2c\xf8" \
// "\x4a\x89\xa8\xc8\xf5\x2c\x14\xe3\x7e\x68\xbd\x42\xb3\x15\x47\xc3" \
// "\x77\xb2\x74\x8b\x95\x1b\x2f"
