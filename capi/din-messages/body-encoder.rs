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

use super::*;
use crate::prelude::v2g::*;
use std::mem;

// export body type to other crate modules
pub type DinBodyType = cglue::din_BodyType;

pub struct V2gMessageHeader {
    payload: cglue::din_MessageHeaderType,
}

impl V2gMessageHeader {
    pub fn new(session_id: &[u8]) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_MessageHeaderType>() };
        payload.SessionID.bytesLen = bytes_to_array(
            session_id,
            &mut payload.SessionID.bytes,
            cglue::din_sessionIDType_BYTES_SIZE,
        )?;
        Ok(Self { payload })
    }

    pub fn set_notification_used(&mut self) -> &mut Self {
        self.payload.set_Notification_isUsed(1);
        self
    }

    pub fn get_notification_used(&self) -> bool {
        if self.payload.Notification_isUsed() == 0 {
            false
        } else {
            true
        }
    }

    pub fn set_signature_used(&mut self) -> &mut Self {
        self.payload.set_Notification_isUsed(1);
        self
    }

    pub fn get_signature_used(&self) -> bool {
        if self.payload.Notification_isUsed() == 0 {
            false
        } else {
            true
        }
    }

    pub fn get_session_id(&self) -> &[u8] {
        let session = array_to_bytes(
            &self.payload.SessionID.bytes,
            self.payload.SessionID.bytesLen,
        );
        session
    }

    pub fn decode(payload: cglue::din_MessageHeaderType) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    pub fn encode(&self) -> cglue::din_MessageHeaderType {
        self.payload
    }
}

pub struct ExiMessageDoc {
    payload: cglue::din_exiDocument,
}

impl ExiMessageDoc {
    #[track_caller]
    pub fn decode_from_stream(locked: &RawStream) -> Result<ExiMessageDoc, AfbError> {
        let payload = unsafe {
            let mut buffer = mem::MaybeUninit::<cglue::din_exiDocument>::uninit();
            let status = cglue::decode_din_exiDocument(locked.stream, buffer.as_mut_ptr());
            let exi_raw = buffer.assume_init();
            if status < 0 {
                return afb_error!(
                    "din-exi-decode",
                    "fail to decode din (ExiDocument) from stream"
                );
            }
            locked.reset();
            exi_raw
        };
        Ok(Self { payload })
    }

    #[track_caller]
    pub fn encode_to_stream(&self, locked: &mut RawStream) -> Result<(), AfbError> {
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

        let status = unsafe {
            cglue::encode_din_exiDocument(
                locked.stream,
                &self.payload as *const _ as *mut cglue::din_exiDocument,
            )
        };
        if status < 0 {
            return afb_error!(
                "exi-iso-encode",
                "fail to encode encode_din_exiDocument to exi"
            );
        }

        // retrieve document encoded size from stream and insert header
        let index = locked.get_cursor() as u32;
        v2gtp20_write_header(
            locked.buffer.as_mut_ptr(),
            index - SDP_V2G_HEADER_LEN as u32,
            V2GTP20_SAP_PAYLOAD_ID,
        );

        // force stream size for get_buffer function
        locked.set_size(index);

        Ok(())
    }

    pub fn new(header: &V2gMessageHeader, body: &DinBodyType) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_exiDocument>() };
        payload.V2G_Message.Header = header.encode();
        payload.V2G_Message.Body = *body;
        Self { payload }
    }

    pub fn get_header(&self) -> V2gMessageHeader {
        V2gMessageHeader::decode(self.payload.V2G_Message.Header)
    }

    #[track_caller]
    pub fn get_body(&self) -> Result<MessageBody, AfbError> {
        MessageBody::decode(&self.payload.V2G_Message.Body)
    }
}
