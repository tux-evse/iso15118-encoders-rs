use iso15118::prelude::iso20_exi::*;
use iso15118::prelude::v2g;
use iso15118::prelude::{dump_buffer, ExiStream};

pub fn encode_to_stream(mut message: ExiMessageDoc) -> Result<ExiStream, AfbError> {
    const SESSION_ID: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    // mock network stream and encode message
    let stream = ExiStream::new();
    {
        let mut lock = stream.lock_stream();
        message.set_session_id(&SESSION_ID);
        message.encode_to_stream(&mut lock)?;
        let doc_size = stream
            .header_check(&lock, v2g::PayloadMsgId::SAP)
            .expect("expect valid V2G header");
        println!("({}) [{}]", doc_size, dump_buffer(lock.get_buffer()));
    }

    Ok(stream)
}

pub fn decode_from_stream(stream: &ExiStream) -> Result<ExiMessageDoc, AfbError> {
    const SESSION_ID: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let mut lock = stream.lock_stream();
    // make sur the stream is rewinded to just after the V2G header
    lock.set_cursor(v2g::SDP_V2G_HEADER_LEN);
    let message = ExiMessageDoc::decode_from_stream(&mut lock)?;

    assert!(message.get_session_id() == SESSION_ID);
    Ok(message)
}

fn encode_then_decode<T>(req: &T) -> Result<T, AfbError>
where
    T: EncodeToDocument + TryFrom<ExiMessageDoc, Error = AfbError>,
{
    let message = ExiMessageDoc::from_payload(req.encode());
    let stream = encode_to_stream(message)?;
    let message2: ExiMessageDoc = decode_from_stream(&stream)?;

    T::try_from(message2)
}

#[test]
fn session_setup_request() -> Result<(), AfbError> {
    // Encoding API
    let evccid_in = "ABCDEF";
    let req = SessionSetupRequest::new(evccid_in)?;

    let req2 = encode_then_decode(&req)?;

    assert!(req2.get_evcc_id() == evccid_in);

    Ok(())
}

#[test]
fn session_setup_response() -> Result<(), AfbError> {
    let code = ResponseCode::OkNewSessionEstablished;
    let evseid_in = "ABCDEF";
    let req = SessionSetupResponse::new(evseid_in, code)?;

    let req2 = encode_then_decode(&req)?;

    assert!(req2.get_evse_id() == evseid_in);
    assert!(req2.get_rcode() == code);

    Ok(())
}

#[test]
fn authorization_setup_response() -> Result<(), AfbError> {
    let mut r = AuthorizationSetupResponse::default();

    let empty_services = Vec::<AuthorizationType>::new();
    assert!(r.set_authorization_services(&empty_services).is_err());

    // array too wide
    let services = vec![
        AuthorizationType::EIM,
        AuthorizationType::PnC,
        AuthorizationType::EIM,
    ];
    assert!(r.set_authorization_services(&services).is_err());

    let services = vec![AuthorizationType::EIM, AuthorizationType::PnC];
    assert!(r.set_authorization_services(&services).is_ok());
    assert!(r.get_authorization_services() == services);

    r.set_eim_authorization_mode(true);
    assert!(r.get_eim_authorization_mode() == true);
    r.set_eim_authorization_mode(false);
    assert!(r.get_eim_authorization_mode() == false);

    // PnC authorization mode
    assert!(r.get_pnc_authorization_mode().is_none());
    assert!(r.set_pnc_authorization_mode(None).is_ok());
    assert!(r.get_pnc_authorization_mode().is_none());

    // empty provider list
    assert!(PnCAuthorizationMode::new([42u8; 16], vec![]).is_ok());

    let pnc_mode = PnCAuthorizationMode::new([42u8; 16], vec![String::from("Provider1")]);
    assert!(pnc_mode.is_ok());
    let pnc_mode = pnc_mode.unwrap();

    assert!(pnc_mode.get_challenge() == [42u8; 16]);
    assert!(pnc_mode.get_provider_list() == vec![String::from("Provider1")]);

    assert!(r.set_pnc_authorization_mode(Some(pnc_mode)).is_ok());
    let read_pnc_mode = r.get_pnc_authorization_mode();
    assert!(read_pnc_mode.is_some());
    let read_pnc_mode = read_pnc_mode.unwrap();
    assert!(read_pnc_mode.is_ok());
    let read_pnc_mode = read_pnc_mode.unwrap();
    assert!(read_pnc_mode.get_challenge() == [42u8; 16]);
    assert!(read_pnc_mode.get_provider_list() == vec![String::from("Provider1")]);

    // after EXI encoding
    let r2 = encode_then_decode(&r)?;
    let read_pnc_mode = r2.get_pnc_authorization_mode();
    assert!(read_pnc_mode.is_some());
    let read_pnc_mode = read_pnc_mode.unwrap();
    assert!(read_pnc_mode.is_ok());
    let read_pnc_mode = read_pnc_mode.unwrap();
    assert!(read_pnc_mode.get_challenge() == [42u8; 16]);
    assert!(read_pnc_mode.get_provider_list() == vec![String::from("Provider1")]);

    Ok(())
}
