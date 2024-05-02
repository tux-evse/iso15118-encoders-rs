use crate::mock_exi::*;
use iso15118::prelude::iso2::*;
use iso15118::prelude::*;

pub fn encode_to_stream<'a>(funcname: &str, body: Iso2BodyType) -> Result<ExiStream, AfbError> {
    const SESSION_ID: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    // mock network stream and encode message
    let stream = ExiStream::new();
    {
        let mut lock = stream.lock_stream();
        let header = Iso2MessageHeader::new(&SESSION_ID)?;
        Iso2MessageDoc::new(&header, &body).encode_to_stream(&mut lock)?;
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

pub fn decode_from_stream(_funcname: &str, stream: ExiStream) -> Result<Iso2MessageDoc, AfbError> {
    let stream_decode = mock_network_input(stream.lock_stream().get_buffer());
    let lock = stream_decode.lock_stream();
    let message = Iso2MessageDoc::decode_from_stream(&lock)?;
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
    let setup_tst = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];
    let payload = SessionSetupRequest::new(&setup_tst)?.encode();

    // encode message to stream_exi an compare with expected binary result
    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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
    let evse_id = "tux-evse-001";
    let rcode = ResponseCode::Ok;
    let payload = iso2::SessionSetupResponse::new(evse_id, rcode)?.encode();

    // encode message to stream_exi an compare with expected binary result
    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::SessionSetupRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let evse_rec = payload.get_id()?;
    let code_rec = payload.get_rcode();
    let _time_stamp = payload.get_time_stamp();

    assert!(evse_id == evse_rec);
    assert!(rcode == code_rec);

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
        .set_category(category_tst)
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x3d, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0xc0, 0x0, 0x8, 0x0, 0x40, 0x29, 0x51, 0xd5, 0xe0, 0xb5, 0x15,
        0xd9, 0xcd, 0x94, 0x4, 0x0, 0x8, 0x80, 0x70, 0x0, 0xa9, 0x8a, 0x88, 0xa1, 0x0, 0x94, 0xe6,
        0x57, 0x47, 0x76, 0xf7, 0x26, 0xb1, 0x0, 0x74, 0x1, 0x53, 0xd5, 0x10, 0x46, 0x1, 0xa, 0xae,
        0xc, 0x8c, 0x2e, 0x8c, 0xa2, 0x20,
    ];

    let rcode = ResponseCode::Ok;
    let mut charging_tst = ServiceCharging::new(1, false);
    charging_tst.set_name("Tux-Evse")?;

    let payment_tst0 = PaymentOption::Contract;
    let payment_tst1 = PaymentOption::External;

    let mut service_tst0 = ServiceOther::new(56, ServiceCategory::Internet, true);
    service_tst0.set_name("LTE")?.set_scope("Network")?;

    let mut service_tst1 = ServiceOther::new(29, ServiceCategory::Other, true);
    service_tst1.set_name("OTA")?.set_scope("Update")?;

    let transfer_tst0 = EngyTransfertMode::AcSinglePhase;
    let transfer_tst1 = EngyTransfertMode::DcBasic;

    let payload = ServiceDiscoveryResponse::new(rcode)
        .set_charging(&charging_tst)
        .add_transfer(transfer_tst0)?
        .add_transfer(transfer_tst1)?
        .add_payment(payment_tst0)?
        .add_payment(payment_tst1)?
        .add_service(&service_tst0)?
        .add_service(&service_tst1)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::ServiceDiscoveryRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_rec = payload.get_rcode();
    let charging_rec = payload.get_charging().unwrap();
    let transfers_rec = payload.get_transfers()?;
    let payments_rec = payload.get_payments();
    let services_rec = payload.get_services()?;

    // assert input==output
    assert!(rcode_rec == rcode);
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

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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
    param_tst0
        .add_param("prm_1", &ParamValue::Int16(123))?
        .add_param("prm_2", &ParamValue::Text("snoopy".to_string()))?
        .add_param(
            "prm_3",
            &ParamValue::PhyValue(PhysicalValue::new(240, 1, PhysicalUnit::Volt)),
        )?;

    let mut param_tst1 = ParamSet::new(2);
    param_tst1
        .add_param("prm_1", &ParamValue::Int16(1234))?
        .add_param("prm_2", &ParamValue::Text("Mme Kermichu".to_string()))?
        .add_param(
            "prm_3",
            &ParamValue::PhyValue(PhysicalValue::new(10, 1, PhysicalUnit::Ampere)),
        )?;

    let id_tst = 56;
    let rcode = ResponseCode::Ok;

    // Encoding api
    let mut payload = ServiceDetailResponse::new(id_tst, rcode);
    payload.add_pset(&param_tst0)?;
    payload.add_pset(&param_tst1)?;

    // keep track of input psets for assert check
    let psets_tst = payload.get_psets();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload.encode())?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::ServiceDetailRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let id_rec = payload.get_id();
    let rcode_rec = payload.get_rcode();
    let psets_rec = payload.get_psets();

    // assert input == output
    assert!(id_tst == id_rec);
    assert!(rcode == rcode_rec);
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
                ParamValue::Int16(rec) => match value_tst {
                    ParamValue::Int16(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_tst:{:?} != value_rec:{}", value_tst, rec),
                },
                ParamValue::Text(rec) => match value_tst {
                    ParamValue::Text(tst) => assert!(rec == tst),
                    _ => panic!("unexpected value_tst:{:?} != value_rec:{}", value_tst, rec),
                },
                ParamValue::PhyValue(rec) => match value_tst {
                    ParamValue::PhyValue(tst) => {
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

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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

    let rcode = ResponseCode::NewSession;
    let processing_tst = EvseProcessing::Finished;

    // Encoding api
    let payload = AuthorizationResponse::new(rcode, processing_tst).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::AuthorizationRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let rcode_rec = payload.get_rcode();
    let processing_rec = payload.get_processing();

    // assert input == output
    assert!(rcode_rec == rcode);
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
    let dc_rcode = DcEvErrorCode::NoError;
    let evresssoc_tst: i8 = 16;
    let status_tst = DcEvStatusType::new(ready_tst, dc_rcode, evresssoc_tst);
    let payload = CableCheckRequest::new(&status_tst).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::CableCheckReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let status_rec = payload.get_status();

    // assert input == output
    assert!(status_rec.get_ready() == ready_tst);
    assert!(status_rec.get_error() == dc_rcode);
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
    let rcode = ResponseCode::NewSession;
    let processing_tst = EvseProcessing::Ongoing;

    let dc_rcode = DcEvseErrorCode::Ready;
    let notification_tst = EvseNotification::ReNegociation;
    let delay_tst = 160;
    let status_tst = DcEvseStatusType::new(dc_rcode, notification_tst, delay_tst);

    let payload = CableCheckResponse::new(rcode, &status_tst, processing_tst).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::CableCheckRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let code_rec = payload.get_rcode();
    let status_rec = payload.get_status();
    let processing_rec = payload.get_processing();

    // assert input == output
    assert!(code_rec == rcode);
    assert!(processing_rec == processing_tst);
    assert!(status_rec.get_notification() == notification_tst);
    assert!(status_rec.get_delay() == delay_tst);
    assert!(status_rec.get_error() == dc_rcode);

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
    let cert0 = CertificateData::new(issuer_tst0, serial_tst0);
    let mut list_tst = CertificateRootList::new(&cert0)?;
    list_tst.add_cert(&CertificateData::new(issuer_tst1, serial_tst1))?;

    let id_tst = "tux-evse";
    let provisioning_tst = [0x1, 0x2, 0x3, 0x4, 0x5, 0x6];

    let payload = CertificateInstallRequest::new(id_tst, &provisioning_tst, &list_tst)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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

    let mut cert_chain_tst = CertificateChainType::new(&cert_main_tst)?;
    cert_chain_tst
        .set_id(cert_id_tst)?
        .add_subcert(&cert_sub_tst0)?
        .add_subcert(&cert_sub_tst1)?;

    let contract_id_tst = "Contract-TuxEvSE";
    let contract_main_tst = [0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6];
    let contract_sub_tst0 = [0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6];
    let contract_sub_tst1 = [0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6];
    let mut contract_chain_tst = CertificateChainType::new(&contract_main_tst)?;
    contract_chain_tst
        .set_id(contract_id_tst)?
        .add_subcert(&contract_sub_tst0)?
        .add_subcert(&contract_sub_tst1)?;

    let private_id_tst = "Private_TuxEvSe";
    let private_data_tst = [0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6];
    let private_key_tst = PrivateKeyType::new(private_id_tst, &private_data_tst)?;

    let public_id_tst = "public_TuxEvSe";
    let public_data_tst = [0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6];
    let public_key_tst = DhPublicKeyType::new(public_id_tst, &public_data_tst)?;

    let emaid_id_tst = "emaid_TuxEvSE";
    let emaid_str_tst = "my emaid testing string";
    let emaid_tst = EmaidType::new(emaid_id_tst, emaid_str_tst)?;

    let rcode = ResponseCode::NewSession;

    let payload = CertificateInstallResponse::new(
        rcode,
        &contract_chain_tst,
        &cert_chain_tst,
        &private_key_tst,
        &public_key_tst,
        &emaid_tst,
    )
    .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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
    assert!(rcode_rec == rcode);
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
        Iso2MessageBody::CurrentDemandReq(msg) => msg,
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

    let payload = iso2::CurrentDemandResponse::new(
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
        Iso2MessageBody::CurrentDemandRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_id().unwrap() == evse_id);
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
        Iso2MessageBody::ChargingStatusReq(msg) => msg,
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

    let payload = iso2::ChargingStatusResponse::new(rcode, evse_id, tuple_id, &ac_status)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::ChargingStatusRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);
    assert!(payload.get_id().unwrap() == evse_id);
    assert!(payload.get_tuple_id() == tuple_id);
    let status = payload.get_ac_evse_status();
    assert!(status.get_notification() == notif);
    assert!(status.get_delay() == delay);
    assert!(status.get_rcd() == rcd);

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

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
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
fn metering_receipt_response() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x17, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x0, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2, 0x0, 0x0,
    ];

    // Encoding API
    let rcode = ResponseCode::Ok;
    let payload = iso2::MeteringReceiptResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::MeteringReceiptRes(msg) => msg,
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
        Iso2MessageBody::ParamDiscoveryReq(msg) => msg,
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
    let dc_params = DcEvChargeParam::new(&dc_status, &dc_max_current, &dc_max_voltage)?;

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
    let dc_params = DcEvChargeParam::new(&dc_status, &dc_max_current, &dc_max_voltage)?;

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
        Iso2MessageBody::ParamDiscoveryReq(msg) => msg,
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
        0x41, 0x81, 0xc2, 0x10, 0xa0, 0x1, 0x0, 0x0, 0x0, 0x4, 0x88, 0x20, 0x78, 0x0, 0x80, 0x0,
        0x48, 0x81, 0x80, 0x50, 0x50, 0x0, 0x0, 0x2, 0x44, 0x10, 0x24, 0x0, 0xc0, 0x0, 0x24, 0x40,
        0xc1, 0x90, 0x2a, 0x8a, 0x0, 0x11, 0x10, 0x82, 0x6, 0x8, 0x0, 0xe2, 0x84, 0x1, 0x90, 0x20,
        0x81, 0xf4, 0x2, 0x8, 0x18, 0x5, 0x2, 0x8, 0x19, 0x0, 0x22, 0x41, 0x0, 0x4, 0x40,
    ];
    // Encoding API
    let rcode = ResponseCode::Ok;
    let processing = EvseProcessing::Ongoing;

    let pmax_a1 = PMaxScheduleEntry::new(1, 2, PhysicalValue::new(240, 1, PhysicalUnit::Volt));
    let pmax_a2 = PMaxScheduleEntry::new(1, 2, PhysicalValue::new(10, 1, PhysicalUnit::Ampere));
    let mut sched_a = SasScheduleTuple::new(1);
    sched_a.add_pmax(&pmax_a1)?.add_pmax(&pmax_a2)?;

    let pmax_b1 = PMaxScheduleEntry::new(1, 2, PhysicalValue::new(400, 1, PhysicalUnit::Volt));
    let pmax_b2 = PMaxScheduleEntry::new(1, 2, PhysicalValue::new(100, 1, PhysicalUnit::Ampere));
    let mut sched_b = SasScheduleTuple::new(1);
    sched_b.add_pmax(&pmax_b1)?.add_pmax(&pmax_b2)?;

    let dc_rcode = DcEvseErrorCode::Ready;
    let dc_notification = EvseNotification::ReNegociation;
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

    let payload = iso2::ParamDiscoveryResponse::new(rcode, processing)
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
        Iso2MessageBody::ParamDiscoveryRes(msg) => msg,
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
        Iso2MessageBody::PaymentDetailsReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_emaid().unwrap() == emaid);

    let contract = payload.get_contract();
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

    let payload = iso2::PaymentDetailsResponse::new(rcode, &challenge)?
        .set_timestamp(0) // force timestamp to get a fix testable buffer
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;

    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::PaymentDetailsRes(msg) => msg,
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
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x19, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x30, 0xd, 0x20, 0x90, 0x70, 0x90, 0x81, 0xc2, 0x42, 0x9, 0x44,
        0xd1, 0x0,
    ];

    let service_contract = PaymentOption::Contract;
    // Encoding API
    let service_option_0 = PaymentServiceOpt::new(1234, Some(4321));
    let service_option_1 = PaymentServiceOpt::new(4321, Some(9876));
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
        Iso2MessageBody::PaymentSelectionReq(msg) => msg,
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
    let payload = iso2::PaymentSelectionResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::PaymentSelectionRes(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_rcode() == rcode);

    Ok(())
}

#[test]
fn power_delivery_request() -> Result<(), AfbError> {
    let expected_response = [
        0x1, 0xfe, 0x80, 0x1, 0x0, 0x0, 0x0, 0x22, 0x80, 0x98, 0x2, 0x0, 0x40, 0x80, 0xc1, 0x1,
        0x41, 0x81, 0xc2, 0x11, 0x52, 0x7, 0xe0, 0x34, 0x82, 0x42, 0xa, 0x8, 0x0, 0x80, 0xd7, 0x23,
        0x8, 0x28, 0x20, 0x1, 0x10, 0x43, 0x8, 0x0, 0x88, 0x0,
    ];

    // Encoding API
    let charge_progress = ChargeProgress::Renegotiate;
    let schedule_id = 64;
    let charge_profile_0 =
        ChargingProfileEntry::new(1234, PhysicalValue::new(64, 1, PhysicalUnit::Watt), Some(3));
    let charge_profile_1 =
        ChargingProfileEntry::new(4567, PhysicalValue::new(64, 1, PhysicalUnit::Watt), Some(2));

    let dc_status = DcEvStatusType::new(true, DcEvErrorCode::FailVoltOutOfRange, 64);
    let dc_delivery_param = DcEvPowerDeliveryParam::new(dc_status, true, Some(true));

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
        Iso2MessageBody::PowerDeliveryReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    assert!(payload.get_progress() == charge_progress);
    assert!(payload.get_schedule_id() == schedule_id);
    let profiles = payload.get_charging_profiles();
    assert!(profiles[0].get_start() == charge_profile_0.get_start());
    assert!(profiles[0].get_phases_max().unwrap() == charge_profile_0.get_phases_max().unwrap());
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

    let payload = iso2::PowerDeliveryResponse::new(rcode)
        .set_ac_evse_status(&ac_status)?
        .encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::PowerDeliveryRes(msg) => msg,
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
        Iso2MessageBody::PreChargeReq(msg) => msg,
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
    let notification = EvseNotification::ReNegociation;
    let delay = 160;
    let dc_status = DcEvseErrorCode::Reserve8;
    let evse_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);
    let mut evse_status = DcEvseStatusType::new(dc_status, notification, delay);
    evse_status.set_isolation_status(IsolationStatus::Warning);

    let payload = iso2::PreChargeResponse::new(rcode, &evse_status, &evse_voltage)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::PreChargeRes(msg) => msg,
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
        Iso2MessageBody::SessionStopReq(msg) => msg,
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
    let payload = iso2::SessionStopResponse::new(rcode).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::SessionStopRes(msg) => msg,
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
    let ready_tst = true;
    let dc_rcode = DcEvErrorCode::NoError;
    let evresssoc_tst: i8 = 16;
    let dc_status = DcEvStatusType::new(ready_tst, dc_rcode, evresssoc_tst);

    let payload = WeldingDetectionRequest::new(&dc_status).encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::WeldingDetectionReq(msg) => msg,
        _ => panic!("Unexpected message type"),
    };

    // Decoding API
    let status_rec = payload.get_status();
    assert!(status_rec.get_ready() == ready_tst);
    assert!(status_rec.get_error() == dc_rcode);
    assert!(status_rec.get_evresssoc() == evresssoc_tst);

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
    let dc_notification = EvseNotification::ReNegociation;
    let dc_delay = 160;
    let dc_status = DcEvseStatusType::new(dc_rcode, dc_notification, dc_delay);
    let dc_voltage = PhysicalValue::new(400, 1, PhysicalUnit::Volt);

    let payload = iso2::WeldingDetectionResponse::new(rcode, &dc_status, &dc_voltage)?.encode();

    // encode message to stream_exi an compare with expected binary result

    let stream = encode_to_stream(func_name!(), payload)?;
    assert!(expected_response == stream.lock_stream().get_buffer());

    // simulate network exi_stream input and decode received message
    let message = decode_from_stream(func_name!(), stream)?;
    let payload = match message.get_body()? {
        Iso2MessageBody::WeldingDetectionRes(msg) => msg,
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
