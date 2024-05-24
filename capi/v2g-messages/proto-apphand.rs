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
use super::status_enums::*;
use super::*;
use std::fmt;
use std::mem;
use std::pin::Pin;

pub type V2gAppHandDoc = cglue::appHand_exiDocument;

// check header and return expected message size (payload+header_size)
pub fn v2gtp_header_check(type_id: PayloadMsgId, buffer: Pin<&[u8]>) -> Result<u32, AfbError> {
    let mut payload_size: u32 = 0;
    let status = unsafe {
        cglue::V2GTP20_ReadHeader(
            buffer.as_ptr(),
            &mut payload_size as *mut u32,
            type_id as u16,
        )
    };
    if status != 0 {
        return afb_error!("v2g-header-check", "invalid payload");
    }

    Ok(payload_size + cglue::V2GTP_HEADER_LENGTH)
}

pub fn v2gtp_get_payload_id(buffer: Pin<&[u8]>) -> PayloadMsgId {
    let payload_id = unsafe { cglue::V2GTP20_GetPayloadId(buffer.as_ptr()) };
    PayloadMsgId::from_u16(payload_id)
}

pub fn v2gtp_get_payload_len(buffer: Pin<&[u8]>) -> i32 {
    let payload_len = unsafe { cglue::V2GTP20_GetPayloadLen(buffer.as_ptr()) };
    payload_len
}

#[inline]
// export cglue function to other create modules
pub(crate) fn v2gtp20_write_header(
    stream_data: *mut u8,
    stream_payload_length: u32,
    v2gtp20_payload_id: u16,
) {
    unsafe { cglue::V2GTP20_WriteHeader(stream_data, stream_payload_length, v2gtp20_payload_id) }
}

#[inline]
#[allow(dead_code)]
// export cglue function to other create modules
pub(crate) fn v2gtp20_read_header(stream_data: *mut u8) -> i32 {
    unsafe { cglue::V2GTP20_GetPayloadLen(stream_data) }
}

#[derive(Debug, Clone)]
pub struct SupportedAppProtocolType {
    pub name_space: String,
    pub version_number_major: u32,
    pub version_number_minor: u32,
    pub schema_id: u8,
    pub priority: u8,
}

#[derive(Clone)]
pub struct AppHandAppProtocolType {
    payload: cglue::appHand_AppProtocolType,
}

impl AppHandAppProtocolType {
    fn new(
        protocol: &SupportedAppProtocolConf,
        schema_id: u8,
        priority: u8,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::appHand_AppProtocolType>() };
        payload.ProtocolNamespace.charactersLen = str_to_array(
            protocol.name,
            &mut payload.ProtocolNamespace.characters,
            cglue::appHand_ProtocolNamespace_CHARACTER_SIZE,
        )?;
        payload.VersionNumberMajor = protocol.major;
        payload.VersionNumberMinor = protocol.minor;
        payload.SchemaID = schema_id;
        payload.Priority = priority;
        Ok(Self { payload })
    }

    pub fn get_name(&self) -> Result<&str, AfbError> {
        let name = array_to_str(
            &self.payload.ProtocolNamespace.characters,
            self.payload.ProtocolNamespace.charactersLen,
        )?;
        Ok(name)
    }

    pub fn get_major(&self) -> u32 {
        self.payload.VersionNumberMajor
    }

    pub fn get_minor(&self) -> u32 {
        self.payload.VersionNumberMinor
    }

    pub fn get_schema(&self) -> u8 {
        self.payload.SchemaID
    }

    pub fn get_priority(&self) -> u8 {
        self.payload.Priority
    }

    pub fn encode(&self) -> cglue::appHand_AppProtocolType {
        self.payload
    }

    pub fn decode(payload: cglue::appHand_AppProtocolType) -> Self {
        Self { payload }
    }
}

impl fmt::Debug for AppHandAppProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name().unwrap();
        let major = self.get_major();
        let minor = self.get_minor();
        let schema = self.get_schema();
        let priority = self.get_priority();
        write!(
            f,
            "(name:{}, major:{} minor:{:?}) schema:{} priority:{}",
            name, major, minor, schema, priority
        )
    }
}

pub struct SupportedAppProtocolReq {
    payload: cglue::appHand_supportedAppProtocolReq,
}

impl SupportedAppProtocolReq {
    pub fn new(protocol: &SupportedAppProtocolConf) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::appHand_supportedAppProtocolReq>() };
        payload.AppProtocol.array[0] = AppHandAppProtocolType::new(protocol, 0, 1)?.encode();
        payload.AppProtocol.arrayLen = 1;
        Ok(Self { payload })
    }

    pub fn decode(payload: cglue::appHand_supportedAppProtocolReq) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> V2gAppHandDoc {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<V2gAppHandDoc>();
            exi_body.__bindgen_anon_1.supportedAppProtocolReq = self.payload;
            exi_body.set_supportedAppProtocolReq_isUsed(1);
            exi_body
        };
        body
    }

    pub fn add_protocol(
        &mut self,
        protocol: &SupportedAppProtocolConf,
    ) -> Result<&mut Self, AfbError> {
        let idx = self.payload.AppProtocol.arrayLen;
        self.payload.AppProtocol.array[idx as usize] =
            AppHandAppProtocolType::new(protocol, 0, 1)?.encode();
        self.payload.AppProtocol.arrayLen += 1;
        Ok(self)
    }

    pub fn get_protocols(&self) -> Vec<AppHandAppProtocolType> {
        let mut response = Vec::new();
        for idx in 0..self.payload.AppProtocol.arrayLen {
            let protocol =
                AppHandAppProtocolType::decode(self.payload.AppProtocol.array[idx as usize]);
            response.push(protocol);
        }
        // sort response by priority before return
        response.sort_by(|a, b| a.get_priority().cmp(&b.get_priority()));
        response
    }

    pub fn match_protocol<'a>(
        &self,
        supported: &[&'a SupportedAppProtocolConf],
    ) -> Result<(ResponseCode, &'a SupportedAppProtocolConf), ResponseCode> {
        let protocols = self.get_protocols();
        for request in protocols {
            for idx in 0..supported.len() {
                let provided = &supported[idx];
                if request.get_name().unwrap() == provided.name
                    && request.get_major() == provided.major
                {
                    afb_log_msg!(Debug, None, "Protocol:{} selected", provided.name);
                    if request.get_minor() == provided.minor {
                        return Ok((ResponseCode::Success, supported[idx]));
                    } else {
                        return Ok((ResponseCode::SuccessWithMinorDeviation, supported[idx]));
                    }
                }
            }
        }
        Err(ResponseCode::Failed)
    }
}

impl fmt::Debug for SupportedAppProtocolReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let protocols = self.get_protocols();
        for protocol in protocols {
            write!(f, "{:?}", protocol)?;
        }
        Ok(())
    }
}

pub struct SupportedAppProtocolRes {
    payload: cglue::appHand_supportedAppProtocolRes,
}

impl SupportedAppProtocolRes {
    pub fn new(code: ResponseCode, schema_id: u8) -> Self {
        {
            let mut payload = unsafe { mem::zeroed::<cglue::appHand_supportedAppProtocolRes>() };
            payload.ResponseCode = code as u32;
            if schema_id != 0 {
                payload.set_SchemaID_isUsed(1);
                payload.SchemaID = schema_id;
            }
            Self { payload }
        }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_schema(&self) -> u8 {
        if self.payload.SchemaID_isUsed() != 0 {
            self.payload.SchemaID
        } else {
            255
        }
    }

    pub fn decode(payload: cglue::appHand_supportedAppProtocolRes) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> V2gAppHandDoc {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<V2gAppHandDoc>();
            exi_body.__bindgen_anon_1.supportedAppProtocolRes = self.payload;
            exi_body.set_supportedAppProtocolRes_isUsed(1);
            exi_body.set_supportedAppProtocolReq_isUsed(0);
            exi_body
        };
        body
    }
}

impl fmt::Debug for SupportedAppProtocolRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "rcode:{} schema:{}", self.get_rcode().to_label(), self.get_schema())?;
        Ok(())
    }
}


pub enum V2gMsgBody {
    Request(SupportedAppProtocolReq),
    Response(SupportedAppProtocolRes),
}

// make following type public to crate
pub struct SupportedAppProtocolExi {}
impl SupportedAppProtocolExi {
    #[track_caller]
    pub fn decode_from_stream(locked: &RawStream) -> Result<V2gMsgBody, AfbError> {
        let body = unsafe {
            let mut exi_raw = mem::MaybeUninit::<cglue::appHand_exiDocument>::uninit();
            let status = cglue::decode_appHand_exiDocument(locked.stream, exi_raw.as_mut_ptr());
            let exi_raw = exi_raw.assume_init();
            if status != 0 {
                return afb_error!(
                    "v2g-exi-decode",
                    "fail to decode v2g (AppProtocolExi) from stream"
                );
            }
            if exi_raw.supportedAppProtocolReq_isUsed() != 0 {
                V2gMsgBody::Request(SupportedAppProtocolReq::decode(
                    exi_raw.__bindgen_anon_1.supportedAppProtocolReq,
                ))
            } else if exi_raw.supportedAppProtocolRes_isUsed() != 0 {
                V2gMsgBody::Response(SupportedAppProtocolRes::decode(
                    exi_raw.__bindgen_anon_1.supportedAppProtocolRes,
                ))
            } else {
                return afb_error!("v2g-exi-decode", "hoops nether request or response");
            }
        };
        Ok(body)
    }

    #[track_caller]
    pub fn encode_to_stream(
        locked: &mut RawStream,
        v2g_body: &V2gAppHandDoc,
    ) -> Result<(), AfbError> {
        locked.reset(); // cleanup stream before encoding

        // reserve space for v2g header
        match unsafe { locked.stream.as_mut() } {
            Some(data) => {
                data.byte_pos = cglue::SDP_V2G_HEADER_LEN as usize;
            }
            None => {
                return afb_error!(
                    "v2g-encode_stream",
                    "fail to get locked.stream (invalid stream)"
                )
            }
        }

        let status = unsafe {
            cglue::encode_appHand_exiDocument(
                locked.stream,
                v2g_body as *const _ as *mut V2gAppHandDoc,
            )
        };
        if status < 0 {
            return afb_error!("v2g-encode_stream", "fail to encode V2gAppHandDoc to exi");
        }

        // retrieve document encoded size from stream
        let index = locked.get_cursor() as u32;
        unsafe {
            cglue::V2GTP20_WriteHeader(
                locked.buffer.as_mut_ptr(),
                index - cglue::SDP_V2G_HEADER_LEN,
                cglue::V2GTP20_SAP_PAYLOAD_ID as u16,
            )
        }
        // force stream size for get_buffer function
        locked.set_size(index);
        Ok(())
    }
}
