use crate::mock_exi::*;
use iso15118::prelude::iso2_exi::*;
use iso15118::prelude::*;

pub fn encode_to_stream<'a>(funcname: &str, body: Iso2BodyType) -> Result<ExiStream, AfbError> {
    const SESSION_ID: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    // mock network stream and encode message
    let stream = ExiStream::new();
    {
        let mut lock = stream.lock_stream();
        let header = ExiMessageHeader::new(&SESSION_ID)?;
        ExiMessageDoc::new(&header, &body).encode_to_stream(&mut lock)?;
        let doc_size = stream
            .header_check(&lock, v2g::PayloadMsgId::SAP)
            .expect("expect valid V2G header");
        println!(
            "{}-> ({}) [{}]",
            funcname,
            doc_size,
            dump_buffer(lock.get_buffer())
        );
    }

    Ok(stream)
}

pub fn decode_from_stream(_funcname: &str, stream: ExiStream) -> Result<ExiMessageDoc, AfbError> {
    let stream_decode = mock_network_input(stream.lock_stream().get_buffer());
    let lock = stream_decode.lock_stream();
    let message = ExiMessageDoc::decode_from_stream(&lock)?;
    Ok(message)
}

#[test]
// cargo test --package iso15118 --test test-v2g --  test_iso2::enum_rename --exact --nocapture
fn enum_rename() {
    println!(
        "MessageTagId:SessionSetupReq= {}",
        MessageTagId::SessionSetupReq.to_label()
    );
    println!(
        "MessageTagId:SessionSetupReq= {}",
        MessageTagId::from_label("session_setup_req").unwrap()
    );
    assert!(MessageTagId::SessionSetupReq.to_label() == "session_setup_req");
    assert!(
        MessageTagId::from_label("session_setup_req").unwrap() == MessageTagId::SessionSetupReq
    );
}
#[test]
fn session_setup_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x15, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xd0, 0x18, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x0,
    ];

    // Encoding API
    let setup_in = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    let payload = SessionSetupRequest::new(&setup_in)?.encode();

    // encode message to stream_exi an compare with expected binary result
    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::SessionSetupReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let setup_out = payload.get_id();
    assert!(setup_in == setup_out);

    Ok(())
}

#[test]
fn session_setup_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x1d, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xe0, 0x0, 0x39, 0xd1, 0xd5, 0xe0, 0xb5, 0x95, 0xd9, 0xcd, 0x94,
        0xb4, 0xc0, 0xc0, 0xc4, 0x0, 0x0,
    ];

    // Encoding API
    let evse_id = "tux-evse-001";
    let rcode = ResponseCode::Ok;
    let payload = SessionSetupResponse::new(evse_id, rcode)?
        .set_time_stamp(0)
        .encode();

    // encode message to stream_exi an compare with expected binary result
    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::SessionSetupRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let evse_out = payload.get_id()?;
    let code_out = payload.get_rcode();
    let _time_stamp = payload.get_time_stamp();

    assert!(evse_id == evse_out);
    assert!(rcode == code_out);

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
    let scope_in = "sample-scope";
    let category_in = ServiceCategory::EvCharger;
    let payload = ServiceDiscoveryRequest::new()
        .set_scope(scope_in)?
        .set_category(category_in)
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ServiceDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let scope_out = payload.get_scope().unwrap();
    let category_out = payload.get_category().unwrap();

    // assert input==output
    assert!(scope_in == scope_out);
    assert!(category_in == category_out);

    Ok(())
}

#[test]
fn service_discovery_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x3d, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xc0, 0x0, 0x8, 0x0, 0x40, 0x29, 0x51, 0xd5, 0xe0, 0xb5, 0x15,
        0xd9, 0xcd, 0x94, 0x4, 0x0, 0x8, 0x80, 0x70, 0x0, 0xa9, 0x8a, 0x88, 0xa1, 0x0, 0x94, 0xe6,
        0x57, 0x47, 0x76, 0xf7, 0x26, 0xb1, 0x0, 0x74, 0x1, 0x53, 0xd5, 0x10, 0x46, 0x1, 0xa, 0xae,
        0xc, 0x8c, 0x2e, 0x8c, 0xa2, 0x20,
    ];

    let rcode = ResponseCode::Ok;
    let mut charging_in = ServiceCharging::new(1, false);
    charging_in.set_name("Tux-Evse")?;

    let payment_in0 = PaymentOption::Contract;
    let payment_in1 = PaymentOption::External;

    let mut service_in0 = ServiceOther::new(56, ServiceCategory::Internet, true);
    service_in0.set_name("LTE")?.set_scope("Network")?;

    let mut service_in1 = ServiceOther::new(29, ServiceCategory::Other, true);
    service_in1.set_name("OTA")?.set_scope("Update")?;

    let transfer_in0 = EngyTransfertMode::AcSinglePhase;
    let transfer_in1 = EngyTransfertMode::DcBasic;

    let payload = ServiceDiscoveryResponse::new(rcode)
        .set_charging(&charging_in)
        .add_transfer(transfer_in0)?
        .add_transfer(transfer_in1)?
        .add_payment(payment_in0)?
        .add_payment(payment_in1)?
        .add_service(&service_in0)?
        .add_service(&service_in1)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ServiceDiscoveryRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_out = payload.get_rcode();
    let charging_out = payload.get_charging().unwrap();
    let transfers_out = payload.get_transfers()?;
    let payments_out = payload.get_payments();
    let services_out = payload.get_services()?;

    // assert input==output
    assert!(rcode_out == rcode);
    assert!(charging_out.get_name() == charging_in.get_name());
    assert!(charging_out.get_scope() == charging_in.get_scope());
    assert!(charging_out.get_isfree() == charging_in.get_isfree());
    assert!(transfers_out[0] == transfer_in0);
    assert!(transfers_out[1] == transfer_in1);
    assert!(payments_out[0] == payment_in0);
    assert!(payments_out[1] == payment_in1);
    assert!(services_out[0].get_id() == service_in0.get_id());
    assert!(services_out[0].get_name() == service_in0.get_name());
    assert!(services_out[0].get_scope() == service_in0.get_scope());
    assert!(services_out[0].get_isfree() == service_in0.get_isfree());
    assert!(services_out[0].get_category() == service_in0.get_category());
    assert!(services_out[1].get_id() == service_in1.get_id());
    assert!(services_out[1].get_name() == service_in1.get_name());
    assert!(services_out[1].get_scope() == service_in1.get_scope());
    assert!(services_out[1].get_isfree() == service_in1.get_isfree());
    assert!(services_out[1].get_category() == service_in1.get_category());

    Ok(())
}

#[test]
fn service_detail_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x10, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x93, 0x48, 0x24, 0x0,
    ];

    let id_in = 1234;

    // Encoding api
    let payload = ServiceDetailRequest::new(id_in).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ServiceDetailReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_out = payload.get_id();

    // assert input == output
    assert!(id_in == id_out);

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

    let mut param_in0 = ParamSet::new(1);
    param_in0
        .add_param(&ParamTuple::new("prm_1", &ParamValue::Int16(123))?)?
        .add_param(&ParamTuple::new(
            "prm_2",
            &ParamValue::Text("snoopy".to_string()),
        )?)?
        .add_param(&ParamTuple::new(
            "prm_3",
            &ParamValue::PhyValue(PhysicalValue::new(240, 1, PhysicalUnit::Volt)),
        )?)?;

    let mut param_in1 = ParamSet::new(2);
    param_in1
        .add_param(&ParamTuple::new("prm_1", &ParamValue::Int16(1234))?)?
        .add_param(&ParamTuple::new(
            "prm_2",
            &ParamValue::Text("Mme Kermichu".to_string()),
        )?)?
        .add_param(&ParamTuple::new(
            "prm_3",
            &ParamValue::PhyValue(PhysicalValue::new(10, 1, PhysicalUnit::Ampere)),
        )?)?;

    let id_in = 56;
    let rcode = ResponseCode::Ok;

    // Encoding api
    let mut payload = ServiceDetailResponse::new(id_in, rcode);
    payload.add_pset(&param_in0)?;
    payload.add_pset(&param_in1)?;

    // keep track of input psets for assert check
    let psets_in = payload.get_psets();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload.encode())?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ServiceDetailRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_out = payload.get_id();
    let rcode_out = payload.get_rcode();
    let psets_out = payload.get_psets();

    // assert input == output
    assert!(id_in == id_out);
    assert!(rcode == rcode_out);
    assert!(psets_out.len() == psets_in.len());
    for idx in 0..psets_out.len() {
        let set_out = psets_out[idx].clone();
        let set_in = psets_in[idx].clone();
        assert!(set_out.get_id() == set_in.get_id());

        let prms_out = set_out.get_params()?;
        let prms_in = set_in.get_params()?;
        assert!(prms_out.len() == prms_in.len());

        for jdx in 0..prms_out.len() {
            assert!(prms_out[jdx].get_name()? == prms_in[jdx].get_name()?);
            let value_out = prms_out[jdx].get_value()?;
            let value_in = prms_in[jdx].get_value()?;

            match value_out {
                ParamValue::Int16(rec) => match value_in {
                    ParamValue::Int16(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_in:{:?} != value_out:{}", value_in, rec),
                },
                ParamValue::Text(rec) => match value_in {
                    ParamValue::Text(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_in:{:?} != value_out:{}", value_in, rec),
                },
                ParamValue::PhyValue(rec) => match value_in {
                    ParamValue::PhyValue(tst) => {
                        assert!(rec.get_unit() == tst.get_unit());
                        assert!(rec.get_multiplier() == tst.get_multiplier());
                        assert!(rec.get_value() == tst.get_value());
                    }
                    _ => panic!("unexpected value_in:{:?} != value_out:{:?}", value_in, rec),
                },
                _ => panic!("unexpected decoded param value:{:?} type", value_out),
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

    let id_in = "tux-evse";
    let challenge_in = [0x1, 0x2, 0x3, 0x4];

    // Encoding api
    let payload = AuthorizationRequest::new()
        .set_id(id_in)?
        .set_challenge(&challenge_in)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::AuthorizationReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // check if authentication is signed
    let header= message.get_header();

    // Decoding API
    let id_out = payload.get_id().unwrap();
    let challenge_out = payload.get_challenge().unwrap();

    // assert input == output
    assert!(id_in == id_out);
    assert!(challenge_in == challenge_out);

    Ok(())
}

#[test]
fn authorization_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xf, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x10, 0x20, 0x0,
    ];

    let rcode = ResponseCode::NewSession;
    let processing_in = EvseProcessing::Finished;

    // Encoding api
    let payload = AuthorizationResponse::new(rcode, processing_in).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::AuthorizationRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_out = payload.get_rcode();
    let processing_out = payload.get_processing();

    // assert input == output
    assert!(rcode_out == rcode);
    assert!(processing_out == processing_in);

    Ok(())
}

#[test]
fn cable_check_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x10, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x31, 0x0, 0x8, 0x0,
    ];

    // Encoding api
    let ready_in = true;
    let dc_rcode = DcEvErrorCode::NoError;
    let evresssoc_in: i8 = 16;
    let status_in = DcEvStatusType::new(ready_in, dc_rcode, evresssoc_in);
    let payload = CableCheckRequest::new(&status_in).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CableCheckReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let status_out = payload.get_status();

    // assert input == output
    assert!(status_out.get_ready() == ready_in);
    assert!(status_out.get_error() == dc_rcode);
    assert!(status_out.get_evresssoc() == evresssoc_in);

    Ok(())
}

#[test]
fn cable_check_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x13, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x40, 0x21, 0x40, 0x2, 0x22, 0x10, 0x40,
    ];

    // Encoding api
    let rcode = ResponseCode::NewSession;
    let processing_in = EvseProcessing::Ongoing;

    let dc_rcode = DcEvseErrorCode::Ready;
    let notification_in = EvseNotification::ReNegotiation;
    let delay_in = 160;
    let status_in = DcEvseStatusType::new(dc_rcode, notification_in, delay_in);

    let payload = CableCheckResponse::new(rcode, &status_in, processing_in).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CableCheckRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let code_out = payload.get_rcode();
    let status_out = payload.get_status();
    let processing_out = payload.get_processing();

    // assert input == output
    assert!(code_out == rcode);
    assert!(processing_out == processing_in);
    assert!(status_out.get_notification() == notification_in);
    assert!(status_out.get_delay() == delay_in);
    assert!(status_out.get_error() == dc_rcode);

    Ok(())
}

#[test]
fn certificate_install_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x39, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x50, 0x53, 0xa3, 0xab, 0xc1, 0x6b, 0x2b, 0xb3, 0x9b, 0x28, 0xc,
        0x2, 0x4, 0x6, 0x8, 0xa, 0xc, 0x0, 0x94, 0x96, 0xf5, 0x42, 0xe6, 0x27, 0xa6, 0x80, 0xd2,
        0x9, 0x0, 0x35, 0x49, 0x95, 0x91, 0xc1, 0x95, 0xcd, 0xac, 0xb9, 0x89, 0xe9, 0xa0, 0x2b,
        0x8b, 0x4, 0x0,
    ];

    // Encoding api
    let issuer_in0 = "IoT.bzh";
    let serial_in0 = 1234;
    let issuer_in1 = "Redpesk.bzh";
    let serial_in1 = 5678;
    let cert0 = IssuerSerialType::new(issuer_in0, serial_in0)?;
    let cert1 = IssuerSerialType::new(issuer_in1, serial_in1)?;

    let mut list_in = CertificateRootList::new(&cert0)?;
    list_in.add_cert(&cert1)?;

    let id_in = "tux-evse";
    let provisioning_in = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];

    let payload = CertificateInstallRequest::new(id_in, &provisioning_in, &list_in)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CertificateInstallReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_out = payload.get_id()?;
    let provisioning_out = payload.get_provisioning();
    let certs_list_out = payload.get_certs_list().get_certs()?;
    let certs_list_in = list_in.get_certs()?;

    // assert input == output
    assert!(id_out == id_in);
    assert!(provisioning_out == provisioning_in);
    assert!(certs_list_out.len() == certs_list_in.len());

    for idx in 0..certs_list_out.len() {
        let cert_out = &certs_list_out[idx];
        let cert_in = &certs_list_in[idx];

        assert!(cert_out.get_issuer()? == cert_in.get_issuer()?);
        assert!(cert_out.get_serial() == cert_in.get_serial());
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
    let cert_id_in = "Cert-TuxEvSE";
    let cert_main_in = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let cert_sub_in0 = [0x11, 0x12, 0x13, 0x14, 0x15, 0x16];
    let cert_sub_in1 = [0x21, 0x22, 0x23, 0x24, 0x25, 0x26];

    let mut cert_chain_in = CertificateChainType::new(&cert_main_in)?;
    cert_chain_in
        .set_id(cert_id_in)?
        .add_subcert(&cert_sub_in0)?
        .add_subcert(&cert_sub_in1)?;

    let contract_id_in = "Contract-TuxEvSE";
    let contract_main_in = [0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6];
    let contract_sub_in0 = [0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6];
    let contract_sub_in1 = [0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6];
    let mut contract_chain_in = CertificateChainType::new(&contract_main_in)?;
    contract_chain_in
        .set_id(contract_id_in)?
        .add_subcert(&contract_sub_in0)?
        .add_subcert(&contract_sub_in1)?;

    let private_id_in = "Private_TuxEvSe";
    let private_data_in = [0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6];
    let private_key_in = PrivateKeyType::new(private_id_in, &private_data_in)?;

    let public_id_in = "public_TuxEvSe";
    let public_data_in = [0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6];
    let public_key_in = DhPublicKeyType::new(public_id_in, &public_data_in)?;

    let emaid_id_in = "emaid_TuxEvSE";
    let emaid_str_in = "my emaid testing string";
    let emaid_in = EmaidType::new(emaid_id_in, emaid_str_in)?;

    let rcode = ResponseCode::NewSession;

    let payload = CertificateInstallResponse::new(
        rcode,
        &contract_chain_in,
        &cert_chain_in,
        &private_key_in,
        &public_key_in,
        &emaid_in,
    )
    .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CertificateInstallRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_out = payload.get_rcode();
    let cert_chain_out = payload.get_provisioning_chain();
    let contract_chain_out = payload.get_contract_chain();
    let private_key_out = payload.get_private_key();
    let public_key_out = payload.get_public_key();
    let emaid_out = payload.get_emaid();

    // assert input == output
    assert!(rcode_out == rcode);
    assert!(cert_chain_out.get_id() == cert_chain_in.get_id());
    assert!(cert_chain_out.get_cert() == cert_chain_in.get_cert());
    assert!(contract_chain_out.get_id() == contract_chain_in.get_id());
    assert!(contract_chain_out.get_cert() == contract_chain_in.get_cert());
    assert!(private_key_out.get_id()? == private_key_in.get_id()?);
    assert!(private_key_out.get_data() == private_key_in.get_data());
    assert!(public_key_out.get_id()? == public_key_in.get_id()?);
    assert!(private_key_out.get_data() == private_key_in.get_data());
    assert!(emaid_out.get_id()? == emaid_in.get_id()?);
    assert!(emaid_out.get_data()? == emaid_in.get_data()?);
    let certs_sub_out = cert_chain_out.get_subcerts();
    assert!(certs_sub_out[0] == cert_sub_in0);
    assert!(certs_sub_out[1] == cert_sub_in1);

    Ok(())
}

#[test]
fn certificate_update_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x64, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x70, 0x53, 0xa3, 0xab, 0xc1, 0x6b, 0x2b, 0xb3, 0x9b, 0x28, 0x12,
        0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x2d, 0x54, 0x75, 0x78, 0x45, 0x76, 0x53,
        0x45, 0x1, 0xa8, 0x68, 0xa8, 0xe9, 0x29, 0x69, 0x80, 0xd, 0x63, 0x65, 0x67, 0x69, 0x6b,
        0x6c, 0x0, 0xd8, 0x38, 0x58, 0x78, 0x98, 0xb8, 0xc4, 0x5, 0xba, 0x3a, 0xbc, 0x16, 0xb2,
        0xb6, 0xb0, 0xb4, 0xb2, 0x0, 0x25, 0x25, 0xbd, 0x50, 0xb9, 0x89, 0xe9, 0xa0, 0x34, 0x82,
        0x40, 0xd, 0x52, 0x65, 0x64, 0x70, 0x65, 0x73, 0x6b, 0x2e, 0x62, 0x7a, 0x68, 0xa, 0xe2,
        0xc1, 0x0,
    ];

    // Encoding api
    let issuer_in0 = "IoT.bzh";
    let serial_in0 = 1234;
    let issuer_in1 = "Redpesk.bzh";
    let serial_in1 = 5678;
    let cert0 = IssuerSerialType::new(issuer_in0, serial_in0)?;
    let mut root_certs = CertificateRootList::new(&cert0)?;
    root_certs.add_cert(&IssuerSerialType::new(issuer_in1, serial_in1)?)?;

    let id_in = "tux-evse";
    let emaid = "tux-emaid";
    let contract_id_in = "Contract-TuxEvSE";
    let contract_main_in = [0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6];
    let contract_sub_in0 = [0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6];
    let contract_sub_in1 = [0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6];
    let mut contract_chain_in = CertificateChainType::new(&contract_main_in)?;
    contract_chain_in
        .set_id(contract_id_in)?
        .add_subcert(&contract_sub_in0)?
        .add_subcert(&contract_sub_in1)?;
    let payload =
        CertificateUpdateRequest::new(id_in, &contract_chain_in, emaid, &root_certs)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CertificateUpdateReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let contract_chain_out = payload.get_contract_chain();
    let root_certs_out = payload.get_root_certs().get_certs()?;

    assert!(id_in == payload.get_id()?);
    assert!(emaid == payload.get_emaid()?);

    let root_certs_in = root_certs.get_certs()?;
    for idx in 0..root_certs_out.len() {
        assert!(root_certs_out[idx].get_issuer()? == root_certs_in[idx].get_issuer()?);
        assert!(root_certs_out[idx].get_serial() == root_certs_in[idx].get_serial());
    }
    assert!(contract_chain_out.get_id() == contract_chain_in.get_id());
    assert!(contract_chain_out.get_cert() == contract_chain_in.get_cert());

    Ok(())
}

#[test]
fn certificate_update_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xb1, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x80, 0x20, 0x1c, 0x86, 0xca, 0xe4, 0xe8, 0x5a, 0xa8, 0xea, 0xf0,
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
        0xe0, 0x3, 0x0,
    ];

    // Encoding api
    let cert_id_in = "Cert-TuxEvSE";
    let cert_main_in = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
    let cert_sub_in0 = [0x11, 0x12, 0x13, 0x14, 0x15, 0x16];
    let cert_sub_in1 = [0x21, 0x22, 0x23, 0x24, 0x25, 0x26];

    let mut cert_chain_in = CertificateChainType::new(&cert_main_in)?;
    cert_chain_in
        .set_id(cert_id_in)?
        .add_subcert(&cert_sub_in0)?
        .add_subcert(&cert_sub_in1)?;

    let contract_id_in = "Contract-TuxEvSE";
    let contract_main_in = [0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6];
    let contract_sub_in0 = [0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6];
    let contract_sub_in1 = [0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6];
    let mut contract_chain_in = CertificateChainType::new(&contract_main_in)?;
    contract_chain_in
        .set_id(contract_id_in)?
        .add_subcert(&contract_sub_in0)?
        .add_subcert(&contract_sub_in1)?;

    let private_id_in = "Private_TuxEvSe";
    let private_data_in = [0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6];
    let private_key_in = PrivateKeyType::new(private_id_in, &private_data_in)?;

    let public_id_in = "public_TuxEvSe";
    let public_data_in = [0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6];
    let public_key_in = DhPublicKeyType::new(public_id_in, &public_data_in)?;

    let emaid_id_in = "emaid_TuxEvSE";
    let emaid_str_in = "my emaid testing string";
    let emaid_in = EmaidType::new(emaid_id_in, emaid_str_in)?;
    let rcount_in = 3;

    let rcode = ResponseCode::NewSession;

    let payload = CertificateUpdateResponse::new(
        rcode,
        &contract_chain_in,
        &cert_chain_in,
        &private_key_in,
        &public_key_in,
        &emaid_in,
    )
    .set_rcount(rcount_in)
    .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CertificateUpdateRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_out = payload.get_rcode();
    let cert_chain_out = payload.get_provisioning_chain();
    let contract_chain_out = payload.get_contract_chain();
    let private_key_out = payload.get_private_key();
    let public_key_out = payload.get_public_key();
    let emaid_out = payload.get_emaid();
    let rcount_out = payload.get_rcount().unwrap();

    // assert input == output
    assert!(rcode_out == rcode);
    assert!(cert_chain_out.get_id() == cert_chain_in.get_id());
    assert!(cert_chain_out.get_cert() == cert_chain_in.get_cert());
    assert!(contract_chain_out.get_id() == contract_chain_in.get_id());
    assert!(contract_chain_out.get_cert() == contract_chain_in.get_cert());
    assert!(private_key_out.get_id()? == private_key_in.get_id()?);
    assert!(private_key_out.get_data() == private_key_in.get_data());
    assert!(public_key_out.get_id()? == public_key_in.get_id()?);
    assert!(private_key_out.get_data() == private_key_in.get_data());
    assert!(emaid_out.get_id()? == emaid_in.get_id()?);
    assert!(emaid_out.get_data()? == emaid_in.get_data()?);
    assert!(rcount_in == rcount_out);
    let certs_sub_out = cert_chain_out.get_subcerts();
    assert!(certs_sub_out[0] == cert_sub_in0);
    assert!(certs_sub_out[1] == cert_sub_in1);

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
    let dc_current = PhysicalValue::new(80, 1, PhysicalUnit::Ampere);
    let dc_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let dc_limit = PhysicalValue::new(800, 1, PhysicalUnit::Volt);
    let dc_complete = true;

    let payload = CurrentDemandRequest::new(&dc_status, &dc_current, &dc_voltage, dc_complete)
        .set_voltage_limit(&dc_limit)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CurrentDemandReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(dc_complete == payload.get_charging_complete());
    let current = payload.get_current_target();
    assert!(current.get_unit() == PhysicalUnit::Ampere);
    assert!(current.get_value() == 80);
    assert!(current.get_multiplier() == 1);
    let voltage = payload.get_voltage_target();
    assert!(voltage.get_value() == 400);
    assert!(payload.get_voltage_limit().unwrap().get_value() == 800);

    Ok(())
}

#[test]
fn current_demand_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x2b, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xe0, 0x1, 0xa4, 0x12, 0x10, 0x40, 0x1, 0x4, 0x9, 0x0, 0x30, 0x40,
        0xc1, 0x0, 0x20, 0x26, 0xe, 0x74, 0x75, 0x78, 0x2d, 0x65, 0x76, 0x73, 0x65, 0x2d, 0x30,
        0x30, 0x31, 0x6, 0xe8,
    ];

    // Encoding API
    let evse_id = "tux-evse-001";
    let rcode = ResponseCode::Ok;
    let voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let current = PhysicalValue::new(64, 1, PhysicalUnit::Ampere);
    let current_limit = true;
    let voltage_limit = false;
    let power_limit = true;
    let schd_tuple_id = 56;

    let dc_rcode = DcEvseErrorCode::NotReady;
    let delay = 1234;
    let notif = EvseNotification::StopCharging;
    let isolation = IsolationStatus::Warning;
    let mut dc_status = DcEvseStatusType::new(dc_rcode, notif, delay);
    dc_status.set_isolation_status(isolation);

    let payload = CurrentDemandResponse::new(
        rcode,
        evse_id,
        &dc_status,
        &current,
        current_limit,
        &voltage,
        voltage_limit,
        power_limit,
        schd_tuple_id,
    )?
    .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::CurrentDemandRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_evse_id().unwrap() == evse_id);
    assert!(payload.get_current_limit_reach() == current_limit);
    assert!(payload.get_voltage_limit_reach() == voltage_limit);
    assert!(payload.get_power_limit_reach() == power_limit);
    assert!(payload.get_tuple_id() == schd_tuple_id);
    let status = payload.get_status();
    assert!(status.get_isolation_status().unwrap() == isolation);
    assert!(status.get_error() == dc_rcode);
    assert!(status.get_notification() == notif);
    assert!(status.get_delay() == delay);

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

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let _payload = match message.get_body()? {
        MessageBody::ChargingStatusReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API (nothing to do)

    Ok(())
}

#[test]
fn charging_status_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x21, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xc0, 0x0, 0x39, 0xd1, 0xd5, 0xe0, 0xb5, 0x95, 0xd9, 0xcd, 0x94,
        0xb4, 0xc0, 0xc0, 0xc4, 0x1f, 0x99, 0xa4, 0x12, 0x11, 0x0,
    ];

    // Encoding API
    let evse_id = "tux-evse-001";
    let rcode = ResponseCode::Ok;
    let tuple_id = 64;

    let rcd = true;
    let delay = 1234;
    let notif = EvseNotification::StopCharging;
    let ac_status = AcEvseStatusType::new(notif, delay, rcd);

    let payload = ChargingStatusResponse::new(rcode, evse_id, tuple_id, &ac_status)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ChargingStatusRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_evse_id().unwrap() == evse_id);
    assert!(payload.get_tuple_id() == tuple_id);
    let status = payload.get_ac_evse_status();
    assert!(status.get_notification() == notif);
    assert!(status.get_delay() == delay);
    assert!(status.get_rcd() == rcd);

    Ok(())
}

#[test]
fn metering_outeipt_request() -> Result<(), AfbError> {
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

    let mut meter_info = MeterInfo::new(meeter_id)?;
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

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::MeteringReceiptReq(msg) => msg,
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
fn metering_outeipt_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x17, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::Ok;
    let payload = MeteringReceiptResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::MeteringReceiptRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);

    Ok(())
}

#[test]
fn ac_param_discovery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x20, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0x90, 0x20, 0x0, 0x1a, 0x41, 0x21, 0x46, 0x1, 0x40, 0x41, 0x2,
        0x40, 0xc, 0x10, 0x30, 0x40, 0x4, 0xc, 0x2, 0x80,
    ];
    let ea_mount = PhysicalValue::new(20, 10, PhysicalUnit::Wh);
    let ac_max_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let ac_max_current = PhysicalValue::new(64, 1, PhysicalUnit::Ampere);
    let ac_min_current = PhysicalValue::new(10, 1, PhysicalUnit::Ampere);
    let mut ac_params =
        AcEvChargeParam::new(&ea_mount, &ac_max_voltage, &ac_max_current, &ac_min_current)?;
    ac_params.set_departure_time(1234);

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::AcSinglePhase)
        .set_max_schedule_tuple(16)
        .set_ac_charge_param(&ac_params)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ParamDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_transfert_energy_mode() == EngyTransfertMode::AcSinglePhase);
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

    let dc_max_voltage = PhysicalValue::new(800, 1, PhysicalUnit::Volt);
    let dc_max_current = PhysicalValue::new(100, 1, PhysicalUnit::Ampere);
    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::NoError, 1);
    let dc_params = DcEvChargeParam::new(&dc_status, &dc_max_voltage, &dc_max_current)?;

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::DcBasic)
        .set_max_schedule_tuple(16)
        .set_dc_charge_param(&dc_params)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ParamDiscoveryReq(msg) => msg,
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

    let ea_mount = PhysicalValue::new(20, 10, PhysicalUnit::Wh);
    let ac_max_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let ac_max_current = PhysicalValue::new(64, 1, PhysicalUnit::Ampere);
    let ac_min_current = PhysicalValue::new(10, 1, PhysicalUnit::Ampere);
    let mut ac_params =
        AcEvChargeParam::new(&ea_mount, &ac_max_voltage, &ac_max_current, &ac_min_current)?;
    ac_params.set_departure_time(1234);

    let dc_max_voltage = PhysicalValue::new(800, 1, PhysicalUnit::Volt);
    let dc_max_current = PhysicalValue::new(100, 1, PhysicalUnit::Ampere);
    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::NoError, 1);
    let dc_params = DcEvChargeParam::new(&dc_status, &dc_max_voltage, &dc_max_current)?;

    let mut charge_param = EvChargeParam::new(&ac_params, &dc_params);
    charge_param.set_departure_time(1234);

    // Encoding API
    let payload = ParamDiscoveryRequest::new(EngyTransfertMode::DcCombo)
        .set_max_schedule_tuple(16)
        .set_ev_charge_param(&charge_param)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ParamDiscoveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_max_schedule_tuple().unwrap() == 16);
    // check DC params
    let params = payload.get_ev_charge_param().unwrap();
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

#[test]
fn param_discovery_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x47, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x10, 0xa0, 0x1, 0x0, 0x0, 0x0, 0x28, 0xf, 0x1, 0x4, 0xf, 0x0, 0x10, 0x0,
        0x18, 0x3c, 0x2, 0x6, 0x1, 0x41, 0x40, 0x0, 0x21, 0x4, 0x9, 0x0, 0x30, 0x21, 0x3, 0x6,
        0x40, 0xaa, 0x28, 0x0, 0x44, 0x42, 0x8, 0x18, 0x20, 0x3, 0x8a, 0x10, 0x6, 0x40, 0x82, 0x7,
        0xd0, 0x8, 0x20, 0x60, 0x14, 0x8, 0x20, 0x64, 0x0, 0x89, 0x4, 0x0, 0x11, 0x0,
    ];
    // Encoding API
    let rcode = ResponseCode::Ok;
    let processing = EvseProcessing::Ongoing;

    let mut pmax_a1 = PMaxScheduleEntry::new(&PhysicalValue::new(240, 1, PhysicalUnit::Volt));
    pmax_a1.set_relative_time_interval(RelativeTimeInterval::new(10).set_duration(60));

    let mut pmax_a2 = PMaxScheduleEntry::new(&PhysicalValue::new(10, 1, PhysicalUnit::Ampere));
    pmax_a2.set_relative_time_interval(RelativeTimeInterval::new(3).set_duration(120));

    let mut sched_a = SasScheduleTuple::new(1);
    sched_a.add_pmax(&pmax_a1)?.add_pmax(&pmax_a2)?;

    let pmax_b1 = PMaxScheduleEntry::new(&PhysicalValue::new(400, 1, PhysicalUnit::Volt));
    let pmax_b2 = PMaxScheduleEntry::new(&PhysicalValue::new(100, 1, PhysicalUnit::Ampere));
    let mut sched_b = SasScheduleTuple::new(1);
    sched_b.add_pmax(&pmax_b1)?.add_pmax(&pmax_b2)?;

    let dc_rcode = DcEvseErrorCode::Ready;
    let dc_notification = EvseNotification::ReNegotiation;
    let dc_delay = 160;
    let dc_status = DcEvseStatusType::new(dc_rcode, dc_notification, dc_delay);
    let max_voltage = PhysicalValue::new(250, 1, PhysicalUnit::Volt);
    let min_voltage = PhysicalValue::new(200, 1, PhysicalUnit::Volt);
    let max_current = PhysicalValue::new(64, 1, PhysicalUnit::Ampere);
    let min_current = PhysicalValue::new(10, 1, PhysicalUnit::Ampere);
    let max_power = PhysicalValue::new(6400, 100, PhysicalUnit::Watt);
    let current_ripple = PhysicalValue::new(1, 1, PhysicalUnit::Volt);
    let charge_param = DcEvseChargeParam::new(
        &dc_status,
        &max_voltage,
        &min_voltage,
        &max_current,
        &min_current,
        &max_power,
        &current_ripple,
    )?;

    let payload = ParamDiscoveryResponse::new(rcode, processing)
        .add_schedule_tuple(&sched_a)?
        .add_schedule_tuple(&sched_b)?
        .set_evse_dc_charge_param(&charge_param)
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::ParamDiscoveryRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_processing() == processing);
    let charge_prm = payload.get_evse_dc_charge_param().unwrap();
    assert!(charge_prm.get_status().get_error() == dc_rcode);
    assert!(charge_prm.get_status().get_notification() == dc_notification);
    assert!(charge_prm.get_status().get_delay() == dc_delay);
    assert!(charge_param.get_max_voltage().get_value() == 250);
    assert!(charge_param.get_min_voltage().get_value() == 200);
    assert!(charge_param.get_max_current().get_value() == 64);
    assert!(charge_param.get_min_current().get_value() == 10);
    assert!(charge_param.get_max_power().get_value() == 6400);
    assert!(charge_param.get_peak_current_ripple().get_value() == 1);

    Ok(())
}

#[test]
fn payment_detail_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x44, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x10, 0x45, 0xd1, 0xd5, 0xe0, 0xb5, 0x95, 0xd9, 0x95, 0xcd, 0x94,
        0xb5, 0x95, 0xb5, 0x85, 0xa5, 0x90, 0x4, 0x1d, 0x1d, 0x5e, 0xb, 0x59, 0x5d, 0x99, 0x5c,
        0xd9, 0x4b, 0x58, 0xd9, 0x5c, 0x9d, 0x0, 0x6a, 0xab, 0xbc, 0xcd, 0xde, 0xef, 0xf0, 0x3,
        0x50, 0xd8, 0xe0, 0xe8, 0xf0, 0xf8, 0x80, 0x35, 0x15, 0x96, 0x16, 0x97, 0x17, 0x91, 0x0,
    ];

    let emaid = "tux-evese-emaid";
    let cert_id = "tux-evese-cert";
    let cert_data = [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let cert_sub_0 = [0xa1, 0xb1, 0xc1, 0xd1, 0xe1, 0xf1];
    let cert_sub_1 = [0xa2, 0xb2, 0xc2, 0xd2, 0xe2, 0xf2];

    let mut cert_chain = CertificateChainType::new(&cert_data)?;
    cert_chain
        .set_id(cert_id)?
        .add_subcert(&cert_sub_0)?
        .add_subcert(&cert_sub_1)?;

    // Encoding API
    let payload = PaymentDetailsRequest::new(&emaid, &cert_chain)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PaymentDetailsReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_emaid().unwrap() == emaid);

    let contract = payload.get_contract_chain();
    assert!(contract.get_id().unwrap() == cert_id);
    assert!(contract.get_cert() == cert_data);
    let subcerts = contract.get_subcerts();
    assert!(subcerts[0] == cert_sub_0);
    assert!(subcerts[1] == cert_sub_1);
    Ok(())
}

#[test]
fn payment_detail_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x18, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x21, 0xe0, 0x1c, 0x24, 0x20, 0x1c, 0x18, 0x14, 0x10, 0xc, 0x0,
        0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::ContractCanceled;
    let challenge = [0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03];

    let payload = PaymentDetailsResponse::new(rcode, &challenge)?
        .set_timestamp(0) // force timestamp to get a fix testable buffer
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;

    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PaymentDetailsRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_challenge() == challenge);
    assert!(payload.get_rcode() == rcode);

    Ok(())
}

#[test]
fn payment_selection_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x18, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x30, 0xd, 0x20, 0x90, 0x3d, 0x81, 0xc2, 0x42, 0xb, 0x70, 0x41,
        0x0,
    ];

    let service_contract = PaymentOption::Contract;
    // Encoding API
    let mut service_option_0 = SelectedService::new(1234);
    service_option_0.set_param_id(123);
    let mut service_option_1 = SelectedService::new(4321);
    service_option_1.set_param_id(567);

    let payload = PaymentSelectionRequest::new(service_contract)
        .add_service(&service_option_0)?
        .add_service(&service_option_1)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PaymentSelectionReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_option() == service_contract);
    let services = payload.get_services();
    assert!(services[0].get_service_id() == service_option_0.get_service_id());
    assert!(services[0].get_param_id() == service_option_0.get_param_id());
    assert!(services[1].get_service_id() == service_option_1.get_service_id());
    assert!(services[1].get_param_id() == service_option_1.get_param_id());

    Ok(())
}

#[test]
fn payment_selection_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xe, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x40, 0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::Ok;
    let payload = PaymentSelectionResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PaymentSelectionRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);

    Ok(())
}

#[test]
fn power_delivery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x20, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x52, 0x7, 0xe0, 0x34, 0x82, 0x42, 0xa, 0x8, 0x2, 0x1a, 0xe4, 0x61,
        0x5, 0x4, 0x1, 0x41, 0xc, 0x20, 0xa, 0x0,
    ];

    // Encoding API
    let charge_progress = ChargeProgress::Renegotiate;
    let schedule_id = 64;
    let charge_profile_0 =
        ChargingProfileEntry::new(1234, &PhysicalValue::new(64, 1, PhysicalUnit::Watt))?;
    let charge_profile_1 =
        ChargingProfileEntry::new(4567, &PhysicalValue::new(64, 1, PhysicalUnit::Watt))?;

    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::FailVoltOutOfRange, 64);
    let dc_delivery_param = DcEvPowerDeliveryParam::new(&dc_status, true);

    let payload = PowerDeliveryRequest::new(charge_progress, schedule_id)
        .add_charging_profile(&charge_profile_0)?
        .add_charging_profile(&charge_profile_1)?
        .set_dc_delivery_params(&dc_delivery_param)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PowerDeliveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_progress() == charge_progress);
    assert!(payload.get_schedule_id() == schedule_id);
    let profiles = payload.get_charging_profiles();
    assert!(profiles[0].get_start() == charge_profile_0.get_start());
    assert!(profiles[0].get_power_max().get_unit() == charge_profile_0.get_power_max().get_unit());
    assert!(
        profiles[0].get_power_max().get_value() == charge_profile_0.get_power_max().get_value()
    );
    assert!(
        profiles[0].get_power_max().get_multiplier()
            == charge_profile_0.get_power_max().get_multiplier()
    );
    let delivery_prm = payload.get_dc_delivery_params().unwrap();
    assert!(delivery_prm.get_status().get_error() == DcEvErrorCode::FailVoltOutOfRange);
    assert!(delivery_prm.get_status().get_ready() == dc_status.get_ready());
    assert!(delivery_prm.get_charge_complete() == true);

    Ok(())
}

#[test]
fn power_ac_delivery_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x12, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x60, 0x0, 0xd2, 0x9, 0x8, 0x80,
    ];

    // Encoding API
    let rcode = ResponseCode::Ok;

    let rcd = true;
    let delay = 1234;
    let notif = EvseNotification::StopCharging;
    let ac_status = AcEvseStatusType::new(notif, delay, rcd);

    let payload = PowerDeliveryResponse::new(rcode)
        .set_ac_evse_status(&ac_status)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PowerDeliveryRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    let status = payload.get_ac_evse_status().unwrap();
    assert!(status.get_notification() == ac_status.get_notification());
    assert!(status.get_delay() == ac_status.get_delay());
    assert!(status.get_rcd() == ac_status.get_rcd());

    Ok(())
}

#[test]
fn pre_charge_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x18, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x71, 0x0, 0x0, 0x82, 0x8, 0x12, 0x0, 0x60, 0x81, 0x82, 0x80, 0x0,
    ];

    let target_current = PhysicalValue::new(80, 1, PhysicalUnit::Ampere);
    let target_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let ev_status = DcEvStatusType::new(true, DcEvErrorCode::NoError, 1);

    // Encoding API
    let payload = PreChargeRequest::new(&ev_status, &target_voltage, &target_current)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PreChargeReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_status().get_error() == DcEvErrorCode::NoError);
    assert!(payload.get_status().get_ready() == true);
    assert!(payload.get_status().get_evresssoc() == 1);
    assert!(payload.get_target_voltage().get_unit() == target_voltage.get_unit());
    assert!(payload.get_target_voltage().get_value() == target_voltage.get_value());
    assert!(payload.get_target_voltage().get_multiplier() == target_voltage.get_multiplier());
    assert!(payload.get_target_current().get_unit() == target_current.get_unit());
    assert!(payload.get_target_current().get_value() == target_current.get_value());
    assert!(payload.get_target_current().get_multiplier() == target_current.get_multiplier());

    Ok(())
}

#[test]
fn pre_charge_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x18, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x80, 0x61, 0x40, 0x2, 0x20, 0x41, 0xc1, 0x4, 0x9, 0x0, 0x30, 0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::CertificateExpiresSoon;
    let notification = EvseNotification::ReNegotiation;
    let delay = 160;
    let dc_status = DcEvseErrorCode::Reserve8;
    let evse_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let mut evse_status = DcEvseStatusType::new(dc_status, notification, delay);
    evse_status.set_isolation_status(IsolationStatus::Warning);

    let payload = PreChargeResponse::new(rcode, &evse_status, &evse_voltage)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::PreChargeRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_status().get_error() == dc_status);
    assert!(payload.get_status().get_isolation_status().unwrap() == IsolationStatus::Warning);
    assert!(payload.get_voltage().get_unit() == evse_voltage.get_unit());
    assert!(payload.get_voltage().get_value() == evse_voltage.get_value());
    assert!(payload.get_voltage().get_multiplier() == evse_voltage.get_multiplier());

    Ok(())
}

#[test]
fn session_stop_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xe, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xf0, 0x0,
    ];

    // Encoding API
    let action = ChargingSessionType::Terminate;
    let payload = SessionStopRequest::new(action).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::SessionStopReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_action() == action);

    Ok(())
}

#[test]
fn session_stop_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0xe, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x12, 0x0, 0x80,
    ];

    // Encoding API
    let rcode = ResponseCode::Failed;
    let payload = SessionStopResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::SessionStopRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);

    Ok(())
}

#[test]
fn welding_detection_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x10, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x12, 0x11, 0x0, 0x8, 0x0,
    ];

    // Encoding API
    let ready_in = true;
    let dc_rcode = DcEvErrorCode::NoError;
    let evresssoc_in: i8 = 16;
    let dc_status = DcEvStatusType::new(ready_in, dc_rcode, evresssoc_in);

    let payload = WeldingDetectionRequest::new(&dc_status).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::WeldingDetectionReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let status_out = payload.get_status();
    assert!(status_out.get_ready() == ready_in);
    assert!(status_out.get_error() == dc_rcode);
    assert!(status_out.get_evresssoc() == evresssoc_in);

    Ok(())
}

#[test]
fn welding_detection_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x17, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x12, 0x20, 0x21, 0x40, 0x2, 0x22, 0x10, 0x41, 0x2, 0x40, 0xc, 0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::NewSession;

    let dc_rcode = DcEvseErrorCode::Ready;
    let dc_notification = EvseNotification::ReNegotiation;
    let dc_delay = 160;
    let dc_status = DcEvseStatusType::new(dc_rcode, dc_notification, dc_delay);
    let dc_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);

    let payload = WeldingDetectionResponse::new(rcode, &dc_status, &dc_voltage)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        MessageBody::WeldingDetectionRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    let voltage = payload.get_voltage();
    assert!(voltage.get_value() == 400);
    assert!(voltage.get_unit() == PhysicalUnit::Volt);
    let status = payload.get_status();
    assert!(status.get_error() == dc_rcode);
    assert!(status.get_delay() == dc_delay);
    assert!(status.get_notification() == dc_notification);
    Ok(())
}
