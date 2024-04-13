use iso15118::prelude::*;
use std::sync::MutexGuard;

// inputy a buffer return a stream with v2g buffer and exi data
pub fn mock_network_input(exi_data: &[u8]) -> ExiStream {
    // create a new stream with attached 8KB buffer
    let stream = ExiStream::new();
    // simulate network data read
    // preempt stream mutex
    // feed stream buffer (server should use zero copy)
    // free stream mutex
    {
        let mut lock = stream.lock_stream();
        lock.buffer[0..exi_data.len()].copy_from_slice(&exi_data);

        // check V2G header (should be donne before finalize to get doc len)
        let doc_size = stream.header_check(&lock).expect("expect valid V2G header");

        // validate buffer stream (should not be locked)
        stream
            .finalize(&lock, doc_size)
            .expect("expect valid stream handle");
    }
    stream
}

pub fn encode_to_stream<'a>(
    funcname: &str,
    stream: &'a ExiStream,
    body: Iso2BodyType,
) -> MutexGuard<'a, RawStream> {
    let net_session = SessionId::new(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], 8);

    // mock network stream and encode message
    let mut stream_lock = stream.lock_stream();
    Iso2MessageExi::encode_to_stream(&mut stream_lock, &body, &net_session).unwrap();

    let length = stream_lock.get_length();
    let buffer = &stream_lock.buffer[0..length];
    println!("{}: [{}]", funcname, dump_buffer(buffer));
    stream_lock
}

pub fn decode_from_stream(stream: &MutexGuard<RawStream>) -> Result<Iso2Payload, AfbError> {
    let stream_decode = mock_network_input(stream.get_buffer());
    let stream_lock = stream_decode.lock_stream();
    let message = Iso2Payload::decode(&stream_lock)?;
    Ok(message)
}
