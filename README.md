# iso15118-encoder-rs

Rust encoding/decoding API for Electric Vehicle protocols. Relies on cbexigen iso15118-encoder library for low level EXI binary encoding and GTLS for signature handling.

Dependencies: https://github.com/tux-evse/iso15118-encoders

## Compiling

Install C dependencies:
```
dnf install libgnutls-devel
git clone https://github.com/tux-evse/iso15118-encoders
mkdir ./iso15118-encoders/build && cd ./iso15118-encoders/build
cmake .. && make install
```

Build without libafb microservice framework
```
cargo build --features=afbmock
```

Build with libafb microservice framework
```
# install libafb dependencies
wget -O - https://raw.githubusercontent.com/redpesk-devtools/redpesk-sdk-tools/master/install-redpesk-sdk.sh | bash

cargo build --features=afbv4
```

## Testing

```
cargo test --package iso15118 --test test-v2g
```

## Encoding/Decoding Api

Currently the only API documentation is provided through the testing suite. Hopefully the API should be easy to integrate with any network stack. While samples leverage Chargebyte exi-stream library to interface with our network TCP/TLS server, the encoders/decoders rely on Rust native '&[u8]' type and do not depend on exi_stream or libafb.



Example of asynchronous network interface to receive an XML/EXI message. As EXI messages may potentially be up to 8KB, they may arrive in chucks. It is the responsibility of developer to assemble message part before calling decoder.

Following sample used exi-stream to assemble message chunks. It call iso-decoder only when full exi message is store into stream buffer. Snippet of code extracted from https://github.com/tux-evse/iso15118-network-rs

```Rust
// New TLS client connecting
AfbEvtFdRegister!(AsyncTlsClientCb, async_tls_client_cb, AsyncTlsClientCtx);
fn async_tls_client_cb(
    _evtfd: &AfbEvtFd,
    revent: u32,
    ctx: &mut AsyncTlsClientCtx,
) -> Result<(), AfbError> {
    if revent != AfbEvtFdPoll::IN.bits() {
        let boxe = unsafe { Box::from_raw(ctx) };
        drop(boxe);
        return Ok(());
    }

   // move tcp socket data into exi stream buffer
    let mut lock = ctx.stream.lock_stream();
    let read_count = {
        let (stream_idx, stream_available) = ctx.stream.get_index(&lock);
        let read_count = if stream_available == 0 {
            afb_log_msg!(
                Notice,
                None,
                "async_tls_client {:?}, buffer full close session",
                ctx.connection.get_source()
            );
            ctx.connection.close()?;
            return Ok(());
        } else {
            let buffer = &mut lock.buffer[stream_idx..];
            ctx.connection.get_data(buffer)?
        };

        // when facing a new exi check how much data should be read
        if stream_idx == 0 {
            ctx.payload_len = ctx.stream.header_check(&lock)?;
            ctx.data_len = 0;
        }
        read_count
    };

    // if data send in chunks let's complete exi buffer before processing it
    ctx.data_len = ctx.data_len + read_count;
    if ctx.data_len == ctx.payload_len {
        ctx.stream.finalize(&lock, ctx.payload_len)?;

        // decode request and encode response
        ctx.controler.handle_exi_doc(&ctx.stream, &mut lock)?;

        // send response and wipe stream for next request
        let response = ctx.stream.get_buffer(&lock);
        ctx.connection.put_data(response)?;
        ctx.stream.reset(&lock);
    }
    Ok(())
}
```

Example of decoding request, encoding response. Stream input contain a full encoded Request and when returnin stream output contain the full encoded response with body, header, signatures,...

```Rust
use iso2::*;
let message = Iso2Payload::decode(stream_in)?;
match message.get_payload() {
    Iso2MessageBody::ServiceDiscoveryReq(request) => {
        let scope = match request.get_scope() {
            Some(value) => value.to_string(),
            None => "no-scope-defined".to_string(),
        };

        afb_log_msg!(Debug, None, "DiscoverySvcReq optional scope:[{}]", scope);
        let charging= ServiceCharging::new("Tux-Evse", "IoT-bzh", false);
        let service = ServiceOther::new(56, "LTE", "Network", ServiceCategory::Internet, true);
        let transfer= EngyTransfertMode::AcSinglePhase;

        let body = ServiceDiscoveryResponse::new(ResponseCode::Ok)
            .add_payment(PaymentOption::Contract)?
            .set_charging(&charging)?
            .add_service(&service)?
            .add_transfer(transfer)?
            .encode();

        Iso2MessageExi::encode_to_stream(stream_out, &body, &session_id)?;
    } // end DiscoverySvcReq
}
...
```