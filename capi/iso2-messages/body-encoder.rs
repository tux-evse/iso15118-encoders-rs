/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use crate::prelude::v2g::*;
use std::mem;
use super::*;

// export body type to other crate modules
pub type Iso2BodyType = cglue::iso2_BodyType;

// session ID is an anonymous function
#[derive(Clone)]
pub struct SessionId {
    handle: ExiByteArray<[u8; cglue::iso2_sessionIDType_BYTES_SIZE as usize]>,
}

impl SessionId {
    pub fn new(id: &[u8; cglue::iso2_sessionIDType_BYTES_SIZE as usize], len: u16) -> Self {
        SessionId {
            handle: ExiByteArray::new(id.clone(), len),
        }
    }

    pub fn null() -> Self {
        Self {
            handle: ExiByteArray::new([0x0; cglue::iso2_sessionIDType_BYTES_SIZE as usize], 0),
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.handle.to_bytes()
    }

    pub fn equal(&self, session: &Self) -> bool {
        self.handle.equal(session.handle.to_bytes())
    }
}
pub struct Iso2MessageExi {
    pub session_id: SessionId,
    pub notification: Option<cglue::iso2_NotificationType>,
    pub signature: Option<cglue::iso2_SignatureType>,
    pub body: Iso2MessageBody,
}

impl Iso2MessageExi {
    #[track_caller]
    pub fn decode_from_stream(locked: &RawStream) -> Result<Iso2MessageExi, AfbError> {
        let exi_raw = unsafe {
            let mut exi_raw = mem::MaybeUninit::<cglue::iso2_exiDocument>::uninit();
            let status = cglue::decode_iso2_exiDocument(locked.stream, exi_raw.as_mut_ptr());
            let exi_raw = exi_raw.assume_init();
            if status < 0 {
                return afb_error!(
                    "iso2-exi-decode",
                    "fail to decode iso-2 (ExiDocument) from stream"
                );
            }
            exi_raw
        };

        let v2g_message = exi_raw.V2G_Message;
        let v2g_header = v2g_message.Header;
        let v2g_body = v2g_message.Body;

        let session_id = SessionId::new(&v2g_header.SessionID.bytes, v2g_header.SessionID.bytesLen);
        let signature = if v2g_header.Signature_isUsed() == 1 {
            Some(v2g_header.Signature)
        } else {
            None
        };

        let notification = if v2g_header.Notification_isUsed() == 1 {
            Some(v2g_header.Notification)
        } else {
            None
        };

        // decode message payload body
        let body = Iso2MessageBody::decode(&v2g_body)?;

        let response = Iso2MessageExi {
            session_id: session_id,
            notification,
            signature,
            body,
        };

        Ok(response)
    }

    #[track_caller]
    pub fn encode_to_stream(
        locked: &mut RawStream,
        iso2_body: &Iso2BodyType,
        session_id: &SessionId,
    ) -> Result<(), AfbError> {
        locked.reset(); // cleanup stream before encoding

        // reserve space for v2g header
        match unsafe { locked.stream.as_mut() } {
            Some(data) => {
                data.byte_pos = SDP_V2G_HEADER_LEN as usize;
            }
            None => {
                return afb_error!(
                    "encode_stream-header",
                    "fail to get locked.stream (invalid stream)"
                )
            }
        };

        // build iso2 message header
        let iso2_header = unsafe {
            let mut header = mem::zeroed::<cglue::iso2_MessageHeaderType>();
            header.set_Notification_isUsed(0);
            header.set_Signature_isUsed(0);

            header.SessionID.bytes = session_id.handle.get_array().clone();
            header.SessionID.bytesLen = session_id.handle.get_len();

            header
        };

        let exi_doc = cglue::iso2_exiDocument {
            V2G_Message: cglue::iso2_V2G_Message {
                Header: iso2_header,
                Body: iso2_body.clone(),
            },
        };

        let status = unsafe { cglue::encode_iso2_exiDocument(locked.stream, &exi_doc) };
        if status < 0 {
            return afb_error!(
                "exi-iso-encode",
                "fail to encode encode_iso2_exiDocument to exi"
            );
        }

        // retrieve document encoded size from stream and insert header
        let index = locked.get_length() as u32;
        v2gtp20_write_header(
            locked.buffer.as_mut_ptr(),
            index - SDP_V2G_HEADER_LEN as u32,
            V2GTP20_SAP_PAYLOAD_ID,
        );

        Ok(())
    }

    pub fn get_session(&self) -> &SessionId {
        &self.session_id
    }
}
