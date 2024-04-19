use iso15118::prelude::*;

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
        lock.buffer[0..exi_data.len()].copy_from_slice(exi_data);
        lock.set_size(exi_data.len() as u32);

        // check V2G header (should be donne before finalize to get doc len)
        let doc_size = stream.header_check(&lock, v2g::PayloadMsgId::SAP).expect("expect valid V2G header");

        // validate buffer stream (should not be locked)
        stream
            .finalize(&lock, doc_size)
            .expect("expect valid stream handle");
    }
    stream
}
