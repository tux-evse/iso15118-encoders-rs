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

pub struct ExiMessageDoc {
    payload: Box<cglue::iso20_exiDocument>,
}

impl ExiMessageDoc {
    pub fn new() -> Self {
        unsafe {
            let mut payload = Box::<cglue::iso20_exiDocument>::new_uninit();
            std::ptr::write_bytes(payload.as_mut_ptr(), 0, 1);
            Self {
                payload: payload.assume_init(),
            }
        }
    }

    pub fn from_payload(payload: Box<cglue::iso20_exiDocument>) -> Self {
        Self { payload }
    }

    #[track_caller]
    pub fn decode_from_stream(locked: &mut RawStream) -> Result<ExiMessageDoc, AfbError> {
        let payload = unsafe {
            let mut buffer = Box::<cglue::iso20_exiDocument>::new_uninit();
            let status = cglue::decode_iso20_exiDocument(locked.stream, buffer.as_mut_ptr());
            let exi_raw = buffer.assume_init();
            if status < 0 {
                return afb_error!(
                    "iso20-exi-decode",
                    "fail to decode iso-20 (ExiDocument) from stream: {}",
                    status
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
            cglue::encode_iso20_exiDocument(
                locked.stream,
                &*self.payload as *const cglue::iso20_exiDocument as *mut cglue::iso20_exiDocument,
            )
        };
        if status < 0 {
            return afb_error!(
                "exi-iso20-encode",
                "fail to encode encode_iso20_exiDocument to exi"
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

    pub fn get_payload(&self) -> &cglue::iso20_exiDocument {
        &self.payload
    }

    pub fn to_body(&self) -> Result<MessageBody, AfbError> {
        unsafe {
            if self.payload.SessionSetupReq_isUsed() != 0 {
                return Ok(MessageBody::SessionSetupReq(
                    SessionSetupRequest::from_cglue(self.payload.__bindgen_anon_1.SessionSetupReq),
                ));
            } else if self.payload.SessionSetupRes_isUsed() != 0 {
                return Ok(MessageBody::SessionSetupRes(
                    SessionSetupResponse::from_cglue(self.payload.__bindgen_anon_1.SessionSetupRes),
                ));
            } else if self.payload.AuthorizationSetupReq_isUsed() != 0 {
                return Ok(MessageBody::AuthorizationSetupReq(
                    AuthorizationSetupRequest::from_cglue(
                        self.payload.__bindgen_anon_1.AuthorizationSetupReq,
                    ),
                ));
            } else if self.payload.AuthorizationSetupRes_isUsed() != 0 {
                return Ok(MessageBody::AuthorizationSetupRes(
                    AuthorizationSetupResponse::from_cglue(
                        self.payload.__bindgen_anon_1.AuthorizationSetupRes,
                    ),
                ));
            }

            println!("== bitfield: {:?}", self.payload._bitfield_1);
        }
        return afb_error!("iso20-exi-message-doc-to-body", "Unsupported body type");
    }

    fn get_header(&self) -> &cglue::iso20_MessageHeaderType {
        // we assume here all messages have a similar memory layout
        unsafe {
            return &self.payload.__bindgen_anon_1.SessionSetupReq.Header;
        }
    }

    fn get_mut_header(&mut self) -> &mut cglue::iso20_MessageHeaderType {
        // we assume here all messages have a similar memory layout
        unsafe {
            return &mut self.payload.__bindgen_anon_1.SessionSetupReq.Header;
        }
    }

    pub fn set_session_id(&mut self, session_id: &[u8]) {
        let mut_header = self.get_mut_header();

        let len = bytes_to_array(
            session_id,
            &mut mut_header.SessionID.bytes,
            cglue::iso20_sessionIDType_BYTES_SIZE,
        )
        .unwrap();
        mut_header.SessionID.bytesLen = len;
    }

    pub fn get_session_id(&self) -> &[u8] {
        let header = self.get_header();
        array_to_bytes(&header.SessionID.bytes, header.SessionID.bytesLen)
    }

    pub fn set_timestamp(&mut self, ts: u64) {
        self.get_mut_header().TimeStamp = ts;
    }

    pub fn get_timestamp(&self) -> u64 {
        self.get_header().TimeStamp
    }
}

pub trait EncodeToDocument {
    fn encode(&self) -> Box<cglue::iso20_exiDocument>;
}

/*impl PkiSignature for ExiMessageDoc {
    fn pki_sign_check(
        &self,
        tagid: iso20_exi::MessageTagId,
        challenge: &[u8],
        pub_key: &PkiPubKey,
    ) -> Result<(), AfbError> {
        use iso20_exi::*;

        /*if self.payload.V2G_Message.Header.Signature_isUsed() == 0 {
            return afb_error!(
                "iso20-pki-sign-check",
                "error: tagid:{} no signature set in exi header",
                tagid.to_label()
            );
        }

        let status = match tagid {
            MessageTagId::AuthorizationReq => unsafe {
                cglue::iso20_sign_check_authorization_req(
                    &self.payload,
                    challenge.as_ptr(),
                    pub_key.get_payload(),
                )
            },
            MessageTagId::MeteringReceiptReq => unsafe {
                cglue::iso20_sign_check_metering_receipt_req(&self.payload, pub_key.get_payload())
            },
            others => {
                return afb_error!(
                    "exi-message-check-signature",
                    "fail iso2-exi document tagid:{} does not implement signature",
                    others.to_label()
                )
            }
        };*/

        if status != 0 {
            return afb_error!(
                "iso20-pki-sign-check",
                "error:{}",
                PkiErrorStatus::from_u32(status).to_label()
            );
        }

        Ok(())
    }

    fn pki_sign_sign(
        &mut self,
        tagid: iso2_exi::MessageTagId,
        priv_key: &PkiPrivKey,
    ) -> Result<(), AfbError> {
        use iso20_exi::*;

        /*et status = match tagid {
            MessageTagId::AuthorizationReq => unsafe {
                cglue::iso2_sign_sign_authorization_req(&mut self.payload, priv_key.get_payload())
            },
            MessageTagId::MeteringReceiptReq => unsafe {
                cglue::iso2_sign_sign_metering_receipt_req(
                    &mut self.payload,
                    priv_key.get_payload(),
                )
            },
            others => {
                return afb_error!(
                    "exi-message-check-signature",
                    "fail iso2-exi document tagid:{} does not implement signature",
                    others.to_label()
                )
            }
        };

        if status != 0 {
            return afb_error!(
                "iso2-pki-sign-sign",
                "error:{}",
                PkiErrorStatus::from_u32(status).to_label()
            );
        }*/

        Ok(())
    }
}*/
